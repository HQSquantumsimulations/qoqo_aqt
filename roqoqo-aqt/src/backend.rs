// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use crate::{call_operation, AqtApi, AqtInstruction};
use reqwest::blocking;
use reqwest::header::{HeaderValue, ACCEPT};
use roqoqo::backends::EvaluatingBackend;
use roqoqo::backends::RegisterResult;
use roqoqo::operations::*;
use roqoqo::registers::{BitOutputRegister, ComplexOutputRegister, FloatOutputRegister};
use roqoqo::RoqoqoBackendError;
use std::collections::HashMap;
use std::env;
use std::{thread, time};

pub type RegisterDefinition = (
    HashMap<String, BitOutputRegister>,
    HashMap<String, FloatOutputRegister>,
    HashMap<String, ComplexOutputRegister>,
);

/// AQT backend
///
/// provides functions to run circuits and measurements on AQT devices.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Backend<T: AqtApi> {
    /// Number of qubits supported by the device
    pub device: T,
    /// Access token for identification with AQT devices
    access_token: String,
}

/// Payload sent to AQT device containing a vector of AqtCircuits
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
struct AqtPayload {
    /// Vector of circuit sent to AQT device
    circuits: Vec<AqtCircuit>,
}

/// Provides the quantum circuit that is to be simulated along with number of qubits used and number of simulation repetitions
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AqtCircuit {
    /// Number of qubits used by AQT device
    number_of_qubits: u32,
    /// Quantum circuit that is to be simulated
    quantum_circuit: Vec<AqtInstruction>,
    /// Number of times the simulation is repeated
    repetitions: u32,
}

/// Schema of post request body sent to AQT device to run the simulation
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AqtRunData {
    /// Name of the job type
    job_type: String,
    /// Custom label given to job
    label: String,
    /// Payload containing the list of quantum circuits that to be simulated on the AQT device
    payload: AqtPayload,
}

/// Schema for response recieved from AQT device server
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AqtRunResponse {
    #[serde(default)]
    job: AqtJob,
    #[serde(default)]
    response: AqtQuerryResponse,
}

impl AqtRunResponse {
    /// Returns current job_id
    pub fn job_id(&self) -> &String {
        &self.job.job_id
    }
    /// Returns status of simulations e.g. "queued", "ongoing", "finished", "cancelled", "error"
    pub fn status(&self) -> &String {
        &self.response.status
    }
    /// Returns error message in the case when status is set to "error"
    pub fn message(&self) -> &String {
        &self.response.message
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
struct AqtJob {
    #[serde(default)]
    job_id: String,
    #[serde(default)]
    job_type: String,
    #[serde(default)]
    label: String,
    #[serde(default)]
    resource_id: String,
    #[serde(default)]
    workspace_id: String,
    #[serde(default)]
    payload: AqtPayload,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct AqtQuerryResponse {
    #[serde(default)]
    status: String,
    #[serde(default)]
    finished_count: u32,
    #[serde(default)]
    message: String,
    #[serde(default)]
    result: HashMap<u32, Vec<Vec<u32>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct AqtResourceDetails {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    r#type: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    available_qubits: u32,
}

impl<T: AqtApi> Backend<T> {
    /// Creates a new AQT backend.
    ///
    /// # Arguments
    ///
    /// `device` - The AQT device the Backend uses to execute operations and circuits.
    ///            At the moment limited to the AQT simulator.
    /// `access_token` - An access_token is required to access AQT hardware and simulators.
    ///                  The access_token can either be given as an argument here or set via the environmental variable `$AQT_ACCESS_TOKEN`
    ///
    /// # Returns
    ///
    /// `Self` - Backend interface for AQT
    pub fn new(device: T, access_token: Option<String>) -> Result<Self, RoqoqoBackendError> {
        let access_token_internal: String = match access_token {
            Some(s) => s,
            None => env::var("AQT_ACCESS_TOKEN").map_err(|_| {
                RoqoqoBackendError::MissingAuthentication {
                    msg: "AQT access token is missing".to_string(),
                }
            })?,
        };

        Ok(Self {
            device,
            access_token: access_token_internal,
        })
    }

    /// Creates an AQT jSON represenstaion of a [roqoqo::Circuit].
    ///
    /// # Arguments
    ///
    /// `circuit` - An iterator over Operations that represents a circuit that is translated
    ///
    /// # Returns
    ///
    /// `String` - JSON representation of a roqoqo circuit
    pub fn to_aqt_json<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation>,
    ) -> Result<String, RoqoqoBackendError> {
        let mut instruction_vec: Vec<AqtInstruction> = Vec::new();
        for op in circuit {
            if let Some(x) = call_operation(op)? {
                instruction_vec.push(x)
            }
        }

        Ok(serde_json::to_string(&instruction_vec).unwrap())
    }

    /// Converts a [roqoqo::Circuit] to `AqtRunData` object that can be sent to AQT device
    /// to process and initialises registers for measurement
    ///
    /// # Arguments
    ///
    /// `circuit` - An iterator over Operations that represents a circuit that is translated
    ///
    /// # Returns
    ///
    ///  `(AqtRunData, RegisterDefinition, String)` - Object of `AqtRunData`, registers, readout for registers
    pub fn convert_circuit_to_aqt_instructions<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation>,
    ) -> Result<(AqtRunData, RegisterDefinition, String), RoqoqoBackendError> {
        let mut bit_registers: HashMap<String, BitOutputRegister> = HashMap::new();
        let mut float_registers: HashMap<String, FloatOutputRegister> = HashMap::new();
        let mut complex_registers: HashMap<String, ComplexOutputRegister> = HashMap::new();

        let mut number_measurements: usize = 0;
        let mut readout: String = "".to_string();
        let mut instruction_vec: Vec<AqtInstruction> = Vec::new();
        for op in circuit {
            match op {
                Operation::PragmaRepeatedMeasurement(o) => {
                    number_measurements = *o.number_measurements();
                    readout.clone_from(o.readout());
                    if let Some(x) = call_operation(op)? {
                        instruction_vec.push(x)
                    }
                }
                Operation::PragmaSetNumberOfMeasurements(o) => {
                    number_measurements = *o.number_measurements();
                    readout.clone_from(o.readout());
                    if let Some(x) = call_operation(op)? {
                        instruction_vec.push(x)
                    }
                }
                Operation::MeasureQubit(o) => {
                    number_measurements = 1;
                    readout.clone_from(o.readout());
                    if let Some(x) = call_operation(op)? {
                        instruction_vec.push(x)
                    }
                }
                Operation::DefinitionBit(def) => {
                    if *def.is_output() {
                        bit_registers.insert(def.name().clone(), Vec::new());
                    }
                    if let Some(x) = call_operation(op)? {
                        instruction_vec.push(x)
                    }
                }
                Operation::DefinitionFloat(def) => {
                    if *def.is_output() {
                        float_registers.insert(def.name().clone(), Vec::new());
                    }
                    if let Some(x) = call_operation(op)? {
                        instruction_vec.push(x)
                    }
                }
                Operation::DefinitionComplex(def) => {
                    if *def.is_output() {
                        complex_registers.insert(def.name().clone(), Vec::new());
                    }
                    if let Some(x) = call_operation(op)? {
                        instruction_vec.push(x)
                    }
                }
                _ => {
                    if let Some(x) = call_operation(op)? {
                        instruction_vec.push(x)
                    }
                }
            }
        }
        let aqt_instruction_circuit = AqtCircuit {
            number_of_qubits: self.device.number_qubits() as u32,
            quantum_circuit: instruction_vec,
            repetitions: number_measurements as u32,
        };
        let data = AqtRunData {
            job_type: "quantum_circuit".to_string(),
            payload: AqtPayload {
                circuits: vec![aqt_instruction_circuit],
            },
            label: "qoqo_aqt_backend".to_string(),
        };

        Ok((
            data,
            (bit_registers, float_registers, complex_registers),
            readout,
        ))
    }
    /// Sends get request to obtain details of the resource for a given resource id
    pub fn get_resource_details(
        &self,
        client: &blocking::Client,
    ) -> Result<AqtResourceDetails, RoqoqoBackendError> {
        let get_resource_details_url = format!(
            "{}resources/{}",
            self.device.remote_host(),
            self.device.id()
        );
        let client_resp = client
            .get(get_resource_details_url)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .bearer_auth(&self.access_token)
            .send()
            .map_err(|e| RoqoqoBackendError::NetworkError {
                msg: format!("{:?}", e),
            })?;
        let status_code = client_resp.status();
        if status_code != reqwest::StatusCode::OK {
            return Err(RoqoqoBackendError::NetworkError {
                msg: format!(
                    "Failed to get resource details. Request to server failed with HTTP status code {:?}",
                    status_code
                ),
            });
        };
        let resource_response: AqtResourceDetails = client_resp
            .json::<AqtResourceDetails>()
            .map_err(|e| RoqoqoBackendError::NetworkError {
                msg: format!("{:?}", e),
            })?;
        Ok(resource_response)
    }
    /// Sends a post request to the AQT device server with `AqtRunData` containing job information and quantum circuits
    pub fn post_job(
        &self,
        client: &blocking::Client,
        data: AqtRunData,
    ) -> Result<AqtRunResponse, RoqoqoBackendError> {
        // Url to post quantum circuit to AQT simulator
        let post_quantum_circuit_url = format!(
            "{}submit/qoqo-integration/{}",
            self.device.remote_host(),
            self.device.id()
        );
        let resp = client
            .post(post_quantum_circuit_url)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .bearer_auth(&self.access_token)
            .json(&data)
            .send()
            .map_err(|e| RoqoqoBackendError::NetworkError {
                msg: format!("{:?}", e),
            })?;
        let status_code = resp.status();
        if status_code != reqwest::StatusCode::OK {
            return Err(RoqoqoBackendError::NetworkError {
                msg: format!(
                    "Failed to post job to server. Request to server failed with HTTP status code {:?}",
                    status_code
                ),
            });
        };
        let run_response: AqtRunResponse =
            resp.json::<AqtRunResponse>()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?;
        Ok(run_response)
    }
    /// Send get request to the AQT device server to obtain the status of the current job and result of the simulation
    pub fn get_result(
        &self,
        client: &blocking::Client,
        job_id: &str,
    ) -> Result<AqtRunResponse, RoqoqoBackendError> {
        // Url to obtain result of simulation from AQT simulator
        let get_result_url = format!("{}result/{}", self.device.remote_host(), job_id);

        let client_resp = client
            .get(get_result_url)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .bearer_auth(&self.access_token)
            .send()
            .map_err(|e| RoqoqoBackendError::NetworkError {
                msg: format!("{:?}", e),
            })?;
        let status_code = client_resp.status();
        if status_code != reqwest::StatusCode::OK {
            return Err(RoqoqoBackendError::NetworkError {
                msg: format!(
                    "Failed to get result from server. Request to server failed with HTTP status code {:?}",
                    status_code
                ),
            });
        }
        let run_response: AqtRunResponse =
            client_resp
                .json::<AqtRunResponse>()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("second {:?}", e),
                })?;

        Ok(run_response)
    }
}

impl<T: AqtApi> EvaluatingBackend for Backend<T> {
    fn run_circuit_iterator<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation>,
    ) -> RegisterResult {
        let client = reqwest::blocking::Client::builder()
            .https_only(self.device.is_https())
            .build()
            .map_err(|x| RoqoqoBackendError::NetworkError {
                msg: format!("could not create https client {:?}", x),
            })?;
        // check device resource
        let aqt_resources_details = self.get_resource_details(&client)?;
        if aqt_resources_details.status != "online" {
            return Err(RoqoqoBackendError::NetworkError {
                msg: "AQT resource is currently ofline".to_string(),
            });
        }
        if aqt_resources_details.available_qubits < self.device.number_qubits() as u32 {
            return Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "Insuffient qubits on backend device. Maximum available qubits: {}.",
                    aqt_resources_details.available_qubits
                ),
            });
        }
        // Convert circuit to aqt instructions
        let (aqt_run_data, all_registers, readout) =
            self.convert_circuit_to_aqt_instructions(circuit)?;
        let (mut bit_registers, float_registers, complex_registers) = all_registers;
        // Send POST request to AQT device
        let run_response = self.post_job(&client, aqt_run_data)?;
        let job_id: &String = run_response.job_id();
        let mut loop_prevention = 0;
        let mut finished: bool = false;
        thread::sleep(time::Duration::from_secs(1));
        while loop_prevention < 100 {
            loop_prevention += 1;
            // Send GET request to AQT evice
            let run_response = self.get_result(&client, job_id)?;

            if run_response.status() == "finished" {
                let querry_response = run_response.response;
                finished = true;
                let measured_results =
                    querry_response
                        .result
                        .get(&0)
                        .ok_or(RoqoqoBackendError::GenericError {
                        msg:
                            "Failed to get measurement due to incorrect retrieval from AQT response"
                                .to_string(),
                    })?;
                for measured in measured_results.iter() {
                    // loop over repetitions
                    for curr in measured.iter() {
                        let binary_representation: Vec<bool> = (0..self.device.number_qubits())
                            .map(|x| {
                                (*curr as usize)
                                    .div_euclid(2_usize.pow(x as u32))
                                    .rem_euclid(2)
                                    == 1_usize
                            })
                            .collect();
                        if let Some(reg) = bit_registers.get_mut(&readout) {
                            reg.push(binary_representation)
                        }
                    }
                }
                break;
            }
            if run_response.status() == "error" {
                return Err(RoqoqoBackendError::NetworkError {
                    msg: format!(
                        "AQT network backend reported error: {}",
                        run_response.message()
                    ),
                });
            }

            if run_response.status() == "cancelled" {
                return Err(RoqoqoBackendError::NetworkError {
                    msg: "AQT network backend reported that the job was cancelled".to_string(),
                });
            }

            thread::sleep(time::Duration::from_secs(20));
        }
        if !finished {
            return Err(RoqoqoBackendError::Timeout {
                msg: "AQT backend timed out after 20s".to_string(),
            });
        }
        Ok((bit_registers, float_registers, complex_registers))
    }
}
