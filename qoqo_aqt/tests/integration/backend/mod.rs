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

//! Integration test for public API of Basis rotation measurement

use pyo3::prelude::*;
use pyo3::Python;
use qoqo::measurements::ClassicalRegisterWrapper;
use qoqo::CircuitWrapper;
use qoqo_aqt::devices;
use qoqo_aqt::BackendWrapper;
use roqoqo::measurements::ClassicalRegister;
use roqoqo::operations;
use roqoqo::Circuit;
use std::env;

#[test]
fn test_creating_backend() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device_type = py.get_type::<devices::SimulatorDeviceWrapper>();
        let device = device_type
            .call1((3,))
            .unwrap()
            .cast_as::<PyCell<devices::SimulatorDeviceWrapper>>()
            .unwrap();
        let backend_type = py.get_type::<BackendWrapper>();
        let _backend = backend_type
            .call1((device, "DUMMY_ACCESS_TOKEN"))
            .unwrap()
            .cast_as::<PyCell<BackendWrapper>>()
            .unwrap();
    });
    if env::var("AQT_ACCESS_TOKEN").is_ok() {
        Python::with_gil(|py| -> () {
            let device_type = py.get_type::<devices::SimulatorDeviceWrapper>();
            let device = device_type
                .call1((3,))
                .unwrap()
                .cast_as::<PyCell<devices::SimulatorDeviceWrapper>>()
                .unwrap();
            let backend_type = py.get_type::<BackendWrapper>();
            let _backend = backend_type
                .call1((device,))
                .unwrap()
                .cast_as::<PyCell<BackendWrapper>>()
                .unwrap();
        })
    } else {
        Python::with_gil(|py| -> () {
            let device_type = py.get_type::<devices::SimulatorDeviceWrapper>();
            let device = device_type
                .call1((3,))
                .unwrap()
                .cast_as::<PyCell<devices::SimulatorDeviceWrapper>>()
                .unwrap();
            let backend_type = py.get_type::<BackendWrapper>();
            let backend = backend_type.call1((device,));
            match backend {
                Err(_) => (),
                _ => panic!("Missing Access Token does not return correct error"),
            }
        })
    }
}

#[test]
fn test_running_circuit() {
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("readout".to_string(), 3, true);
    circuit += operations::RotateX::new(0, 0.0.into());
    circuit += operations::RotateY::new(0, 1.0.into());
    circuit += operations::RotateZ::new(0, 2.0.into());
    circuit += operations::MolmerSorensenXX::new(0, 1);
    circuit += operations::PragmaRepeatedMeasurement::new("readout".to_string(), None, 100);
    let circuit_wrapper = CircuitWrapper { internal: circuit };
    if env::var("AQT_ACCESS_TOKEN").is_ok() {
        Python::with_gil(|py| -> () {
            let device_type = py.get_type::<devices::SimulatorDeviceWrapper>();
            let device = device_type
                .call1((3,))
                .unwrap()
                .cast_as::<PyCell<devices::SimulatorDeviceWrapper>>()
                .unwrap();
            let backend_type = py.get_type::<BackendWrapper>();
            let backend = backend_type
                .call1((device,))
                .unwrap()
                .cast_as::<PyCell<BackendWrapper>>()
                .unwrap();
            let _ = backend
                .call_method1("run_circuit", (circuit_wrapper,))
                .unwrap();
        })
    }
}

#[test]
fn test_running_measurement() {
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("readout".to_string(), 3, true);
    circuit += operations::RotateX::new(0, 0.0.into());
    circuit += operations::RotateY::new(0, 1.0.into());
    circuit += operations::RotateZ::new(0, 2.0.into());
    circuit += operations::MolmerSorensenXX::new(0, 1);
    circuit += operations::PragmaRepeatedMeasurement::new("readout".to_string(), None, 100);
    let cr_measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![circuit],
    };
    let crm_wrapper = ClassicalRegisterWrapper {
        internal: cr_measurement,
    };
    if env::var("AQT_ACCESS_TOKEN").is_ok() {
        Python::with_gil(|py| -> () {
            let device_type = py.get_type::<devices::SimulatorDeviceWrapper>();
            let device = device_type
                .call1((3,))
                .unwrap()
                .cast_as::<PyCell<devices::SimulatorDeviceWrapper>>()
                .unwrap();
            let backend_type = py.get_type::<BackendWrapper>();
            let backend = backend_type
                .call1((device,))
                .unwrap()
                .cast_as::<PyCell<BackendWrapper>>()
                .unwrap();
            let _ = backend
                .call_method1("run_measurement_registers", (crm_wrapper,))
                .unwrap();
        })
    }
}
