# A quick Tutorial how to use qoqo_aqt

qoqo_aqt provides a backend to the AQT quantum devices and simulators for the qoqo quantum computing toolkit by HQS.

[qoqo is hosted on github](https://github.com/HQSquantumsimulations/qoqo)

[![Documentation Status](https://readthedocs.org/projects/qoqo/badge/?version=latest)](https://qoqo.readthedocs.io/en/latest/?badge=latest)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo)](https://pypi.org/project/qoqo/)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo)](https://pypi.org/project/qoqo/)
[![Crates.io](https://img.shields.io/crates/v/roqoqo)](https://crates.io/crates/qoqo)
![Crates.io](https://img.shields.io/crates/l/qoqo)


## Installation

On macOS on x86, precompiled packages can be found on PyPi and installed via

```shell
pip install qoqo
pip install qoqo_aqt
```

At the moment an issue with building manylinux python wheels with openssl support prevents us from distributing precompiled packages for linux. 
For now please use the same method as for other platforms to install qoqo_aqt on linux.

For other platforms we recommend checking out the latest tagged version from github and using the [maturin](https://github.com/PyO3/maturin) tool to build a python package for qoqo locally and install it via pip.
Please note that the package should be built from the top level directory of the workspace selecting the qoqo package with the `-m qoqo/Cargo.toml` option.
Specifically for macOS on Apple Silicon the following build command should be used:

```shell
cd qoqo
RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup" maturin build -m qoqo/Cargo.toml  --release
pip install target/wheels/$NAME_OF_WHEEL
cd ../qoqo_aqt
RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup" maturin build -m qoqo/Cargo.toml  --release
pip install target/wheels/$NAME_OF_WHEEL
```

## Use your AQT credentials

qoqo_aqt can only be used with valid AQT credentials.
The AQT-Token can either be provided in code when creating the backend 
```python
from qoqo_aqt import devices, Backend

device = devices.SimulatorDevice(number_qubits=2)
backend = Backend(device=device, access_token='MY_TOKEN')

```

or it can be set via an environmental variable `$AQT_ACCESS_TOKEN`

```shell
export AQT_ACCESS_TOKEN=MY_TOKEN
```

```python
from qoqo_aqt import devices, Backend

device = devices.SimulatorDevice(number_qubits=2)
backend = Backend(device=device, access_token=None)
```

You can find out more about AQT subscriptions at the [AQT Gateway portal](https://gateway-portal.aqt.eu/)

## Running a circuit

You can define a circuit in qoqo and run it on a qoqo device.
For example a circuit to create the bell state of two qubits

```
|bell-state> = 1/sqrt(2)(|00> + i |11>)
```

and measure the two qubits can be defined with:

```python
from qoqo import Circuit
from qoqo import operations as ops

circuit = Circuit()
circuit += ops.DefinitionBit("readout", length=2, is_output=True)  # Classical register for readout
circuit += ops.MolmerSorensenXX(control=0, target=1)  # Quantum operations
circuit += ops.PragmaRepeatedMeasurement(readout="readout", number_measurements=100)  # Measuring qubits
```

The circuit can be run with:

```python
from qoqo_aqt import devices, Backend

device = devices.SimulatorDevice(number_qubits=2)
backend = Backend(device=device, access_token='MY_TOKEN')

(bit_registers, float_registers, complex_registers) = backend.run_circuit(circuit)
print(bit_registers["readout"])
```

**Note:** While the AQT backend at the moment returns a measurement of all qubits at the end of the circuit, `PragmaRepeatedMeasurement` is used to set the number of times the circuit is repeated with `number_measurements`.

## Devices

At the moment qoqo_aqt supports two devices `SimulatorDevice` and `NoisySimulatorDevice`.

The first calls the noisy free simulator

```
url = "https://gateway.aqt.eu/marmot/sim/"
```

The second one calls a simulator with a noise model

```
url = "https://gateway.aqt.eu/marmot/sim/noise-model-1"
```

## License
qoqo and qoqo_aqt are licensed under the Apache License 2.0.
