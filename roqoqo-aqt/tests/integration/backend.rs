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

use roqoqo::prelude::*;
use roqoqo::{operations::*, Circuit};
use roqoqo_aqt::devices::SimulatorDevice;
use roqoqo_aqt::Backend;
use roqoqo_test::prepare_monte_carlo_gate_test;
use std::env;

#[test]
fn init_backend() {
    if env::var("AQT_ACCESS_TOKEN").is_ok() {
        let device = SimulatorDevice::new(2);
        let _backend = Backend::new(device.into(), None).unwrap();
    } else {
        let device = SimulatorDevice::new(2);
        let ok = Backend::new(device.into(), None).is_err();
        assert!(ok);
        let device = SimulatorDevice::new(2);
        let ok = Backend::new(device.into(), Some("dummy_access_token".to_string())).is_ok();
        assert!(ok);
    }
}

#[test]
fn run_simple_circuit() {
    if env::var("AQT_ACCESS_TOKEN").is_ok() {
        let device = SimulatorDevice::new(2);
        let backend = Backend::new(device.into(), None).unwrap();
        let mut circuit = Circuit::new();
        circuit += DefinitionBit::new("ro".to_string(), 2, true);
        circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
        circuit += PauliX::new(1);
        circuit += PragmaRepeatedMeasurement::new("ro".to_string(), None, 20);
        let (bit_registers, _float_registers, _complex_registers) =
            backend.run_circuit(&circuit).unwrap();
        assert!(bit_registers.contains_key("ro"));
        let out_reg = bit_registers.get("ro").unwrap();
        assert_eq!(out_reg.len(), 20);
        for reg in out_reg.iter() {
            assert_eq!(reg.len(), 2);
        }
    }
}

/// Simply test measurement process, not that gate is translated correclty
#[test]
#[ignore = "Takes too long and puts large load on AQT servers"]
fn test_measurement() {
    if env::var("AQT_ACCESS_TOKEN").is_ok() {
        let gate: GateOperation = PauliZ::new(0).into();
        let preparation_gates: Vec<SingleQubitGateOperation> = vec![PauliX::new(0).into()];
        let basis_rotation_gates: Vec<SingleQubitGateOperation> = vec![PauliY::new(0).into()];
        let (measurement, exp_vals) = prepare_monte_carlo_gate_test(
            gate,
            preparation_gates,
            basis_rotation_gates,
            None,
            1,
            200,
        );
        let device = SimulatorDevice::new(2);
        let backend = Backend::new(device.into(), None).unwrap();
        let measured_exp_vals = backend.run_measurement(&measurement).unwrap().unwrap();
        for (key, val) in exp_vals.iter() {
            assert!((val - measured_exp_vals.get(key).unwrap()).abs() < 1.0);
        }
    }
}

/// Test full gate with stochastic application of gates, ignore normally because of length and load
#[test]
#[ignore = "Takes too long and puts large load on AQT servers"]
fn test_full_simple_gate() {
    if env::var("AQT_ACCESS_TOKEN").is_ok() {
        let gate: GateOperation = PauliX::new(0).into();
        let preparation_gates: Vec<SingleQubitGateOperation> = vec![
            PauliX::new(0).into(),
            PauliY::new(0).into(),
            PauliZ::new(0).into(),
        ];
        let basis_rotation_gates: Vec<SingleQubitGateOperation> = vec![
            PauliX::new(0).into(),
            RotateX::new(0, std::f64::consts::FRAC_PI_2.into()).into(),
            RotateZ::new(0, std::f64::consts::FRAC_PI_2.into()).into(),
        ];
        let (measurement, exp_vals) = prepare_monte_carlo_gate_test(
            gate,
            preparation_gates,
            basis_rotation_gates,
            None,
            5,
            200,
        );

        let device = SimulatorDevice::new(2);
        let backend = Backend::new(device.into(), None).unwrap();
        let measured_exp_vals = backend.run_measurement(&measurement).unwrap().unwrap();
        for (key, val) in exp_vals.iter() {
            assert!((val - measured_exp_vals.get(key).unwrap()).abs() < 1.0);
        }
    }
}
