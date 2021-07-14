# How to contribute

We are happy to include your contribution to this project. To contribute open a pull request and we will get back to you.

## Contributor License Agreement

To clarify the intellectual property license granted with Contributions from any person or entity to HQS, we must have a Contributor License Agreement ("CLA") in place with each contributor. This license is for your protection as a Contributor as well as the protection of HQS and the users of this project; it does not change your rights to use your own Contributions for any other purpose.

Please fill and sign the CLA found at *url* and send it to info@quantumsimulations.de.

## Code Guidelines for Rust

1. Testing: We use `cargo test` (in qoqo_mock/roqoqo_mock) for roqoqo_mock and `pytest ./tests` (in qoqo_mock/qoqo_mock) for qoqo_mock. We require that all previous tests pass and that your provide proper tests with your contribution.
2. Linting: We use `cargo clippy -- -D warnings` to lint all Rust code (qoqo_mock/roqoqo_mock), and `flake8 ./qoqo_mock`, `mypy ./qoqo_mock` to lint all Python code (qoqo_mock/qoqo_mock).
3. Formatting: We check formatting with `cargo fmt --all --check` in Rust code (qoqo_mock/roqoqo_mock), and with flake8 in Python code (qoqo_mock/qoqo_mock).
