// Copyright © 2021-2025 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use qoqo_aqt::devices;
use test_case::test_case;

#[test_case(1; "1")]
#[test_case(3; "3")]
fn test_creating_device(number_qubits: usize) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<devices::AqtDeviceWrapper>();
        let binding = device_type.call1((number_qubits,)).unwrap();
        let device = binding.downcast::<devices::AqtDeviceWrapper>().unwrap();

        let get_number_qubits = device
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        let remote_host = device
            .call_method0("remote_host")
            .unwrap()
            .extract::<String>()
            .unwrap();
        let is_https = device
            .call_method0("is_https")
            .unwrap()
            .extract::<bool>()
            .unwrap();
        assert_eq!(number_qubits, get_number_qubits);
        assert_eq!(remote_host.as_str(), "https://arnica.aqt.eu/api/v1/");
        assert!(is_https);
    })
}
