// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
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

#![deny(missing_docs)]
#![deny(missing_crate_level_docs)]
#![deny(missing_debug_implementations)]

//! Qoqo quantum computing toolkit
//!
//! Quantum Operation Quantum Operation
//! Yes we use [reduplication](https://en.wikipedia.org/wiki/Reduplication)

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

/// Collection of devices for AQT Backend of qoqo
///
/// AQT devices, provide necessary information about the endpoint the Circuits are run on.
///
/// .. autosummary::
///     :toctree: generated/
///
///     SimulatorDevice
///
pub mod devices;
use devices::*;

mod backend;
pub use backend::*;

/// Quantum Operation Quantum Operation (qoqo)
///
/// Yes, we use reduplication.
///
/// qoqo is the HQS python package to represent quantum circuits.
///
/// .. autosummary::
///     :toctree: generated/
///
///     Backend
///     devices
///
#[pymodule]
fn qoqo_aqt(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<BackendWrapper>()?;
    let wrapper = wrap_pymodule!(devices);
    module.add_wrapped(wrapper)?;
    // Adding nice imports corresponding to maturin example
    let system = PyModule::import(_py, "sys")?;
    let system_modules: &PyDict = system.getattr("modules")?.downcast()?;
    system_modules.set_item("qoqo_aqt.devices", module.getattr("devices")?)?;
    Ok(())
}
