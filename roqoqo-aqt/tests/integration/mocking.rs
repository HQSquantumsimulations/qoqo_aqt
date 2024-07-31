use httpmock::prelude::*;
use reqwest;
use tokio;

use roqoqo::prelude::*;
use roqoqo::{operations::*, Circuit};
use roqoqo_aqt::Backend;
use serde_json::json;
use std::env;

use roqoqo_aqt::{AqtApi, AqtDevice};

#[test]
fn mock_test() {
    // Start a lightweight mock server.

    struct MockAqtDevice {
        pub number_qubits: usize,
        pub mock_host: String,
    }

    impl AqtApi for MockAqtDevice {
        fn remote_host(&self) -> String {
            self.mock_host.to_string()
        }

        fn number_qubits(&self) -> usize {
            self.number_qubits
        }

        fn is_https(&self) -> bool {
            false
        }
    }

    let server = MockServer::start();
    let mock_device = MockAqtDevice {
        number_qubits: 1,
        mock_host: server.url("/h/"),
    };
    let backend = Backend::new(mock_device, Some("Dummy".to_string())).unwrap();

    let mock_post = server.mock(|when, then| {
        when.method(POST)
            .path("/h/submit/qoqo-integration/simulator_noise")
            .header("content-type", "application/json")
            .header("accept", "application/json")
            .json_body(json!({
                    "job_type": "quantum_circuit",
                    "label": "qoqo_aqt_backend",
                    "payload": {
                        "circuits": [
                        {
                            "number_of_qubits": 1,
                            "quantum_circuit": [
                            {
                                "operation": "R",
                                "phi": 0.0,
                                "qubit": 0,
                                "theta": 1.0,
                            },
                            {
                                "operation": "MEASURE"
                            }
                            ],
                            "repetitions": 5
                        }
                        ]
                    }
                    }
            ));
        then.status(200)
            .header("content-type", "text/html")
            .json_body(json!({
                "job": {
                "job_type": "quantum_circuit",
                "label": "mock_example",
                "job_id": "mock-id",
                "resource_id": "simulator_noise",
                "workspace_id": "qoqo-integration"
                },
                "response": {
                    "status": "queued"
                }
            }));
    });

    let mock_get = server.mock(|when, then| {
        when.method(GET)
            .path("/h/result/mock-id")
            .header("accept", "application/json");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
              "job": {
                "job_type": "quantum_circuit",
                "label": "Example computation",
                "job_id": "0f03bf9c-b1c4-4202-a7fa-ae896ba6ab02",
                "resource_id": "",
                "workspace_id": ""
              },
              "response": {
                "status": "finished",
                "result": {
                  "0": [
                    [
                      1
                    ],
                    [
                      1
                    ],
                    [
                      1
                    ],
                    [
                      1
                    ],
                    [
                      1
                    ],
                  ]
                }
              }
            }));
    });

    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 1, true);
    circuit += PauliX::new(0);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 5, None);
    let (_bit_registers, _float_registers, _complex_registers) =
        backend.run_circuit(&circuit).unwrap();

    // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
    mock_post.assert();
    mock_get.assert();
    // Ensure the mock server did respond as specified.
}

#[test]
fn actual_test() {
    // Start a lightweight mock server.

    let device = AqtDevice { number_qubits: 1 };
    let backend = Backend::new(device, None).unwrap();

    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 1, true);
    circuit += PauliX::new(0);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 5, None);
    let (_bit_registers, _float_registers, _complex_registers) =
        backend.run_circuit(&circuit).unwrap();

    println!("{:?}", _bit_registers);

    assert!(false)
    // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
    // Ensure the mock server did respond as specified.
}
