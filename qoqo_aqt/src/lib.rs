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
#![allow(clippy::borrow_deref_ref)]
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

pub mod devices;

mod backend;
pub use backend::{convert_into_backend, BackendWrapper};

/// AQT python interface
///
/// Provides the devices that are used to execute quantum programs with the AQT backend, as well as the AQT backend.
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

    let wrapper = wrap_pymodule!(devices::aqt_devices);
    module.add_wrapped(wrapper)?;

    // Adding nice imports corresponding to maturin example
    let system = PyModule::import(_py, "sys")?;
    let system_modules: &PyDict = system.getattr("modules")?.downcast()?;
    system_modules.set_item("qoqo_aqt.devices", module.getattr("aqt_devices")?)?;
    Ok(())
}
