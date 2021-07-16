# A quick Tutorial how to use roqoqo-aqt

roqoqo-aqt provides a backend to the AQT quantum devices and simulators for the qoqo quantum computing toolkit by HQS.

[roqoqo is hosted on github](https://github.com/HQSquantumsimulations/qoqo)

[![Crates.io](https://img.shields.io/crates/v/roqoqo)](https://crates.io/crates/roqoqo)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo/actions)
[![docs.rs](https://img.shields.io/docsrs/roqoqo)](https://docs.rs/roqoqo/)
![Crates.io](https://img.shields.io/crates/l/roqoqo)


## Installation

To use roqoqo-aqt in a rust project simply add the following line to your Cargo.toml `[dependencies]` section.

```toml
roqoqo-aqt = {version="0.1"}
```

## Use your AQT credentials

roqoqo-aqt can only be used with valid AQT credentials.
The AQT-Token can either be provided in code when creating the backend 
```rust
use roqoqo_aqt::{devices, Backend};

let device = devices::SimulatorDevice::new(2); //number qubits
let backend = Backend::new(device, Some("MY_TOKEN".to_string()))?;
```

or it can be set via an environmental variable `$AQT_ACCESS_TOKEN`

```shell
export AQT_ACCESS_TOKEN=MY_TOKEN
```

```rust
use roqoqo::backends::EvaluatingBackend;
use roqoqo_aqt::{devices, Backend};

let device = devices::SimulatorDevice::new(2); //number qubits
let backend = Backend::new(device.into(), None).unwrap();

    
```

You can find out more about AQT subscriptions at the [AQT Gateway portal](https://gateway-portal.aqt.eu/)

## Running a circuit

You can define a circuit in qoqo and run it on a qoqo device.
For example a circuit to create the bell state of two qubits

```
|bell-state> = 1/sqrt(2)(|00> + i |11>)
```

and measure the two qubits can be defined with:

```rust
use roqoqo::operations;
use roqoqo::Circuit;

let mut circuit = Circuit::new();
circuit += operations::DefinitionBit::new("readout".to_string(), 2, true); // Classical register for readout
circuit += operations::MolmerSorensenXX::new(0, 1); // Quantum operation
circuit += operations::PragmaRepeatedMeasurement::new("readout".to_string(), None, 100); // Measuring qubits
```

The circuit can be run with:

```rust
use roqoqo::backends::EvaluatingBackend;
use roqoqo_aqt::{devices, Backend};

let device = devices::SimulatorDevice::new(2); //number qubits
let backend = Backend::new(device.into(), None).unwrap();

let (bit_registers, _float_registers, _complex_registers) = backend
    .run_circuit(&circuit)
    .expect("Running the circuit failed");
println!("{:?}", bit_registers["readout"]);
```

**Note:** While the AQT backend at the moment returns a measurement of all qubits at the end of the circuit, `PragmaRepeatedMeasurement` is used to set the number of times the circuit is repeated.

## Devices

At the moment roqoqo-aqt supports two devices `SimulatorDevice` and `NoisySimulatorDevice`.

The first calls the noisy free simulator

```
url = "https://gateway.aqt.eu/marmot/sim/"
```

The second one calls a simulator with a noise model

```
url = "https://gateway.aqt.eu/marmot/sim/noise-model-1"
```

## License
roqoqo and roqoqo-aqt are licensed under the Apache License 2.0.
