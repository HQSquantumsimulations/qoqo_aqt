# qoqo-aqt

AQT-backend for the qoqo/roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

The qoqo_aqt/roqoqo-aqt packages provide backends for qoqo/roqoqo that allow the users to run quantum circuits on AQT simulators or AQT hardware.
AQT endpoints (simulators or hardware) are accessed via a web-interface.
To run circuits with these backends the user needs a valid access token for the AQT services and an internet connection.

This repository contains two components:

* The qoqo_aqt backend for the qoqo python interface to roqoqo
* The roqoqo-aqt backend for roqoqo directly

## qoqo_aqt

[![Documentation Status](https://readthedocs.org/projects/qoqo_aqt/badge/?version=latest)](https://qoqo_aqt.readthedocs.io/en/latest/?badge=latest)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_aqt/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_aqt/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo_aqt)](https://pypi.org/project/qoqo_aqt/)
![PyPI - License](https://img.shields.io/pypi/l/qoqo_aqt)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo_aqt)](https://pypi.org/project/qoqo_aqt/)

AQT-backend for the qoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).



### Installation

On Linux and macOS on x86 precompiled packages can be found on PyPi and installed via

```shell
pip install qoqo_aqt
```

For other platforms we recommend checking out the latest tagged version from github and using the [maturin](https://github.com/PyO3/maturin) tool to build a python package for qoqo locally and install it via pip.
Please note that the package should be built from the top level directory of the workspace selecting the qoqo package with the `-m qoqo/Cargo.toml` option.
Specifically for macOS on Apple Silicon the following build command should be used.

```shell
RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup" maturin build -m qoqo_aqt/Cargo.toml  --release
pip install target/wheels/$NAME_OF_WHEEL
```

## roqoqo-aqt

[![Crates.io](https://img.shields.io/crates/v/roqoqo-aqt)](https://crates.io/crates/roqoqo-aqt)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_mock/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_aqt/actions)
[![docs.rs](https://img.shields.io/docsrs/roqoqo-aqt)](https://docs.rs/roqoqo-aqt/)
![Crates.io](https://img.shields.io/crates/l/roqoqo-aqt)

AQT-Backend for the roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

## General Notes

This software is still in the beta stage. Functions and documentation are not yet complete and breaking changes can occur.

## Contributing

We welcome contributions to the project. If you want to contribute code, please have a look at CONTRIBUTE.md for our code contribution guidelines.