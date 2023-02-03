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

//! Qoqo quantum operations for quantum computers
//!
//! Quantum programs are represented by linear sequences of quantum operations

mod simulator_device;
pub use simulator_device::*;

mod noisy_simulator_device;
pub use noisy_simulator_device::*;

use pyo3::prelude::*;

/// AQT Devices
// ///
// /// Provides the devices that are used to execute quantum programs with the AQT backend.
// /// AQT devices can be physical hardware or simulators.
// ///
// /// .. autosummary::
// ///    :toctree: generated/
// ///
// ///    SimulatorDevice
// ///    NoisySimulatorDevice
// ///
#[pymodule]
pub fn aqt_devices(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SimulatorDeviceWrapper>()?;
    m.add_class::<NoisySimulatorDeviceWrapper>()?;
    Ok(())
}
