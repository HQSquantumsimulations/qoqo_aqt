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

use roqoqo_aqt::{devices::AqtDevice, AqtApi};

// Test the functions of the trait AqtApi
#[test]
fn test_aqt_api() {
    let device = AqtDevice::new(2);
    assert_eq!(device.number_qubits(), 2);
    assert!(device.is_https());
    assert_eq!(
        device.remote_host(),
        "https://arnica.aqt.eu/api/v1/".to_string()
    )
}
