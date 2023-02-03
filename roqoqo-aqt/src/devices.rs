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

//! AQT Devices
//!
//! Provides the devices that are used to execute quantum programs with the AQT backend.
//! AQT devices can be physical hardware or simulators.

/// Collection of AQT quantum devices
///
/// At the moment only supports a simulator endpoint
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub enum AqtDevice {
    /// AQT online simulator
    SimulatorDevice(SimulatorDevice),
    /// AQT online simulator with noise model
    NoisySimulatorDevice(NoisySimulatorDevice),
}

impl AqtDevice {
    /// Returns the number of qubits in the device.
    pub fn number_qubits(&self) -> usize {
        match self {
            AqtDevice::SimulatorDevice(x) => x.number_qubits(),
            AqtDevice::NoisySimulatorDevice(x) => x.number_qubits(),
        }
    }

    /// Returns the remote_host url endpoint of the device
    pub fn remote_host(&self) -> &str {
        match self {
            AqtDevice::SimulatorDevice(x) => x.remote_host(),
            AqtDevice::NoisySimulatorDevice(x) => x.remote_host(),
        }
    }
}

impl From<&SimulatorDevice> for AqtDevice {
    fn from(input: &SimulatorDevice) -> Self {
        Self::SimulatorDevice(input.clone())
    }
}

impl From<SimulatorDevice> for AqtDevice {
    fn from(input: SimulatorDevice) -> Self {
        Self::SimulatorDevice(input)
    }
}

/// AQT quantum simulator device
///
/// Provides endpoint that receives instructions that are simulated and returns measurement results.
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimulatorDevice {
    /// Number of qubits supported by the device
    pub number_qubits: usize,
}

impl SimulatorDevice {
    /// Create new simulator device
    ///
    /// # Arguments
    ///
    /// `number_qubits` - Number of qubits that should be simulated
    pub fn new(number_qubits: usize) -> Self {
        Self { number_qubits }
    }

    /// Returns the number of qubits in the device.
    pub fn number_qubits(&self) -> usize {
        self.number_qubits
    }

    /// Returns the remote_host url endpoint of the device
    pub fn remote_host(&self) -> &str {
        "https://gateway.aqt.eu/marmot/sim/"
    }
}

/// AQT noisy quantum simulator device
///
/// Provides endpoint that receives instructions that are simulated and returns measurement results.
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct NoisySimulatorDevice {
    /// Number of qubits supported by the device
    pub number_qubits: usize,
}

impl NoisySimulatorDevice {
    /// Create new simulator device
    ///
    /// # Arguments
    ///
    /// `number_qubits` - Number of qubits that should be simulated
    pub fn new(number_qubits: usize) -> Self {
        Self { number_qubits }
    }

    /// Returns the number of qubits in the device.
    pub fn number_qubits(&self) -> usize {
        self.number_qubits
    }

    /// Returns the remote_host url endpoint of the device
    pub fn remote_host(&self) -> &str {
        "https://gateway.aqt.eu/marmot/sim/noise-model-1"
    }
}

impl From<&NoisySimulatorDevice> for AqtDevice {
    fn from(input: &NoisySimulatorDevice) -> Self {
        Self::NoisySimulatorDevice(input.clone())
    }
}

impl From<NoisySimulatorDevice> for AqtDevice {
    fn from(input: NoisySimulatorDevice) -> Self {
        Self::NoisySimulatorDevice(input)
    }
}
