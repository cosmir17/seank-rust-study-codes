### 6. `rust_tdd_example_study_2`

This module is replicated from the blog post ["Rust Unit and Integration Testing in RustRover"](https://blog.jetbrains.com/rust/2024/04/02/rust-unit-and-integration-testing-in-rustrover) and demonstrates a structured approach to unit and integration testing in Rust.

The project is organised into multiple crates:

- `main`: The binary crate that serves as the entry point of the application.
- `process`: A library crate containing the core functionality and modules.
- `integration_tests`: A library crate dedicated to integration tests.

The `process` crate contains the following modules:

- `methods.rs`: Implements various calculator operations such as addition, subtraction, multiplication, and division.
- `lib.rs`: Defines the public API of the crate, including the `execute` function that processes command-line arguments.

Unit tests are placed within the `process` crate, close to the implementation of the functions being tested. The tests cover different scenarios, including successful operations and failure cases like division by zero.

Integration tests are located in the `integration_tests` crate. The `main_tests.rs` file contains integration tests that exercise the public API of the `process` crate, specifically the `execute` function. These tests ensure that the interaction between the `main` and `process` crates works as expected.

To run the tests, you can use the following commands:

```bash
# Run unit tests
cargo test --package process

# Run integration tests
cargo test --package integration_tests

# Run every test
cargo test --all