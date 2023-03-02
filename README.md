<img src="qoqo_Logo_vertical_color.png" alt="qoqo logo" width="300" />

# qoqo-aqt

AQT-backend for the qoqo/roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

The qoqo_aqt/roqoqo-aqt packages provide backends for qoqo/roqoqo that allow the users to run quantum circuits on AQT simulators or AQT hardware.
AQT endpoints (simulators or hardware) are accessed via a web-interface.
To run circuits with these backends the user needs a valid access token for the AQT services and an internet connection.

This repository contains two components:

* The qoqo_aqt backend for the qoqo python interface to roqoqo
* The roqoqo-aqt backend for roqoqo directly

## qoqo_aqt

[![Documentation Status](https://img.shields.io/badge/docs-documentation-green)](https://hqsquantumsimulations.github.io/qoqo_aqt/)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_aqt/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_aqt/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo_aqt)](https://pypi.org/project/qoqo_aqt/)
![PyPI - License](https://img.shields.io/pypi/l/qoqo_aqt)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo_aqt)](https://pypi.org/project/qoqo_aqt/)

AQT-backend for the qoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

A short tutorial can be found in Tutorial_qoqo.md in the git repository.

### Installation

On  macOS on x86 precompiled packages can be found on PyPi and installed via

```shell
pip install qoqo_aqt
```

At the moment an issue with building manylinux python wheels prevents us from distributing precompiled packages for linux.
For now please use the same method as for other platforms to install qoqo_aqt on linux.

For other platforms we recommend checking out the latest tagged version from github and using the [maturin](https://github.com/PyO3/maturin) tool to build a python package for qoqo locally and install it via pip.
Please note that the package should be built from the top level directory of the workspace selecting the qoqo package with the `-m qoqo/Cargo.toml` option.
Specifically for macOS on Apple Silicon the following build command should be used.

```shell
RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup" maturin build -m qoqo_aqt/Cargo.toml  --release
pip install target/wheels/$NAME_OF_WHEEL
```

A source distribution now exists but requires a Rust install with a rust version > 1.56 and a maturin version { >= 0.14, <0.15 } in order to be built.

## roqoqo-aqt

[![Crates.io](https://img.shields.io/crates/v/roqoqo-aqt)](https://crates.io/crates/roqoqo-aqt)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_mock/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_aqt/actions)
[![docs.rs](https://img.shields.io/docsrs/roqoqo-aqt)](https://docs.rs/roqoqo-aqt/)
![Crates.io](https://img.shields.io/crates/l/roqoqo-aqt)

AQT-Backend for the roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

A short tutorial can be found in Tutorial_roqoqo.md in the git repository.

## General Notes

This software is still in the beta stage. Functions and documentation are not yet complete and breaking changes can occur.

This project is partly supported by [PlanQK](https://planqk.de).

## Contributing

We welcome contributions to the project. If you want to contribute code, please have a look at CONTRIBUTE.md for our code contribution guidelines.

## OpenSSL

Acknowledgments related to using OpenSSL for http requests:

"This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit (http://www.openssl.org/)."

This product includes cryptographic software written by Eric Young
(eay@cryptsoft.com).  This product includes software written by Tim
Hudson (tjh@cryptsoft.com).