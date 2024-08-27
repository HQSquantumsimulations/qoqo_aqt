// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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

//! AQT Device
//!
//! Provides the device used to execute quantum programs with the AQT backend.

/// AQT device
///
/// Consists of information about the device such as the id, the number of qubits, and the endpoint that receives instructions that
/// are simulated and returns measurement results.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AqtDevice {
    /// Number of qubits supported by the device
    pub number_qubits: usize,
}

impl AqtDevice {
    /// Create a new AQT device for the backend
    pub fn new(number_qubits: usize) -> Self {
        Self { number_qubits }
    }
}

impl AqtApi for AqtDevice {
    /// Returns REST API endpoint to make calls to the AQT device
    fn remote_host(&self) -> String {
        "https://arnica.aqt.eu/api/v1/".to_string()
    }
    /// Return number of qubits available
    fn number_qubits(&self) -> usize {
        self.number_qubits
    }
    /// Returns whether the internal client sends request to an https server
    fn is_https(&self) -> bool {
        true
    }
    /// Returns the id of the device
    fn id(&self) -> String {
        "simulator_noise".to_string()
    }
}

/// Defines the AQT backend on which to run quantum simulations
pub trait AqtApi {
    /// Returns REST API endpoint to make calls to the AQT device
    fn remote_host(&self) -> String;
    /// Return number of qubits available
    fn number_qubits(&self) -> usize;
    /// Returns whether the internal client sends request to an https server
    fn is_https(&self) -> bool;
    /// Returns the id of the device
    fn id(&self) -> String;
}
