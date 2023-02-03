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

use crate::devices::AqtDevice;
use crate::{call_operation, AqtInstruction};
use roqoqo::backends::EvaluatingBackend;
// use roqoqo::measurements::Measure;
use roqoqo::backends::RegisterResult;
use roqoqo::operations::*;
use roqoqo::registers::{BitOutputRegister, ComplexOutputRegister, FloatOutputRegister};
use roqoqo::RoqoqoBackendError;
use std::collections::HashMap;
use std::env;
use std::{thread, time};
/// AQT backend
///
/// provides functions to run circuits and measurements on AQT devices.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Backend {
    /// Device the backend calls to run circuits remotely
    pub device: AqtDevice,
    // Access token for identification with AQT devices
    access_token: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct AqtRunData {
    data: String,
    access_token: String,
    repetitions: usize,
    no_qubits: usize,
    label: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AqtRunResponse {
    id: String,
    #[serde(default)]
    status: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct AqtResultQuerry {
    id: String,
    access_token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AqtResultQuerryResponse {
    #[serde(default)]
    id: String,
    #[serde(default)]
    no_qubits: usize,
    #[serde(default)]
    received: Vec<AqtInstruction>,
    #[serde(default)]
    samples: Vec<usize>,
    #[serde(default)]
    status: String,
}

impl Backend {
    /// Creates a new AQT backend.
    ///
    /// # Arguments
    ///
    /// `device` - The AQT device the Backend uses to execute operations and circuits.
    ///            At the moment limited to the AQT simulator.
    /// `access_token` - An access_token is required to access AQT hardware and simulators.
    ///                  The access_token can either be given as an argument here or set via the environmental variable `$AQT_ACCESS_TOKEN`
    pub fn new(
        device: AqtDevice,
        access_token: Option<String>,
    ) -> Result<Self, RoqoqoBackendError> {
        let access_token_internal: String = match access_token {
            Some(s) => s,
            None => env::var("AQT_ACCESS_TOKEN").map_err(|_| {
                RoqoqoBackendError::MissingAuthentification {
                    msg: "AQT access token is missing".to_string(),
                }
            })?,
        };

        Ok(Self {
            device,
            access_token: access_token_internal,
        })
    }

    /// Creates an AQT json represenstaion of a [roqoqo::Circuit].
    ///
    /// # Arguments
    ///
    /// `circuit` - An iterator over Operations that represents a circuit that is translated
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
}

impl EvaluatingBackend for Backend {
    fn run_circuit_iterator<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation>,
    ) -> RegisterResult {
        let mut bit_registers: HashMap<String, BitOutputRegister> = HashMap::new();
        let mut float_registers: HashMap<String, FloatOutputRegister> = HashMap::new();
        let mut complex_registers: HashMap<String, ComplexOutputRegister> = HashMap::new();

        let mut instruction_vec: Vec<AqtInstruction> = Vec::new();
        let client = reqwest::blocking::Client::builder()
            .https_only(true)
            .build()
            .map_err(|x| RoqoqoBackendError::NetworkError {
                msg: format!("could not create https client {:?}", x),
            })?;
        let mut number_measurements: usize = 0;
        let mut readout: String = "".to_string();
        for op in circuit {
            match op {
                Operation::PragmaRepeatedMeasurement(o) => {
                    number_measurements = *o.number_measurements();
                    readout = o.readout().clone();
                    if let Some(x) = call_operation(op)? {
                        instruction_vec.push(x)
                    }
                }
                Operation::PragmaSetNumberOfMeasurements(o) => {
                    number_measurements = *o.number_measurements();
                    readout = o.readout().clone();
                    if let Some(x) = call_operation(op)? {
                        instruction_vec.push(x)
                    }
                }
                Operation::MeasureQubit(o) => {
                    readout = o.readout().clone();
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
        let data = AqtRunData {
            data: serde_json::to_string(&instruction_vec).unwrap(),
            access_token: self.access_token.clone(),
            repetitions: number_measurements,
            no_qubits: self.device.number_qubits(),
            label: "custom".to_string(),
        };
        let resp = client
            .put(self.device.remote_host())
            .header("Ocp-Apim-Subscription-Key", self.access_token.clone())
            .form(&data)
            .send()
            .map_err(|e| RoqoqoBackendError::NetworkError {
                msg: format!("{:?}", e),
            })?;
        let status_code = resp.status();
        if status_code != reqwest::StatusCode::OK {
            return Err(RoqoqoBackendError::NetworkError {
                msg: format!(
                    "Request to server failed with HTTP status code {:?}",
                    status_code
                ),
            });
        }
        let response: AqtRunResponse =
            resp.json::<AqtRunResponse>()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?;

        let querry = AqtResultQuerry {
            id: response.id,
            access_token: self.access_token.clone(),
        };
        let mut loop_prevention = 0;
        let mut finished: bool = false;
        while loop_prevention < 100 {
            loop_prevention += 1;
            let querry_resp = client
                .put(self.device.remote_host())
                .header("Ocp-Apim-Subscription-Key", self.access_token.clone())
                .form(&querry)
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?;
            let status_code = querry_resp.status();
            if status_code != reqwest::StatusCode::OK {
                return Err(RoqoqoBackendError::NetworkError {
                    msg: format!(
                        "Request to server failed with HTTP status code {:?}",
                        status_code
                    ),
                });
            }
            let querry_response: AqtResultQuerryResponse = querry_resp
                .json::<AqtResultQuerryResponse>()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("second {:?}", e),
                })?;
            if querry_response.status.as_str() == "finished" {
                finished = true;
                for measured in querry_response.samples.iter() {
                    let binary_representation: Vec<bool> = (0..self.device.number_qubits())
                        .map(|x| {
                            measured.div_euclid(2_usize.pow(x as u32)).rem_euclid(2) == 1_usize
                        })
                        .collect();
                    if let Some(reg) = bit_registers.get_mut(&readout) {
                        reg.push(binary_representation)
                    }
                }
                break;
            }
            if querry_response.status.as_str() == "error" {
                return Err(RoqoqoBackendError::NetworkError {
                    msg: "AQT network backend reported error".to_string(),
                });
            }
            thread::sleep(time::Duration::from_secs(50));
        }
        if !finished {
            return Err(RoqoqoBackendError::Timeout {
                msg: "AQT backend timed out after 50s".to_string(),
            });
        }
        Ok((bit_registers, float_registers, complex_registers))
    }
}
