# gRPC Client & Mock Server Project

## Introduction

This repository contains a gRPC client project implemented in Rust.

## Technologies Used
This project uses the Rust language and the following libraries:
- Tonic: A Rust gRPC library
- Tokio: An asynchronous runtime for Rust

## Repository Structure

### 1. `Cargo.toml`

This is a Rust build file containing required dependencies and the 'client' binary setup.

### 2. `build.rs`

This is a Tonic-specific build script that runs automatically as part of the Rust build process.

### 3. `src/client.rs`

This file contains the main client implementation.

### 4. `proto/service.proto`

This is the protocol buffer definition file for the gRPC service.

## How To Run

1. Ensure that the token and endpoint used in the code are valid by testing with the grpcurl command. (e.g., grpcurl -plaintext -proto ./proto/service.proto -H "Authorization: Bearer TOKEN" -d @ -v 'example.com:3001' service.Controller/Connection)
2. Run 'cargo build' in the project root directory to build the project.
3. Run 'RUST_LOG=debug cargo run --bin client' to start the client.
4. Use ctrl-c to cancel the execution.

Note:
1. The client is designed to run continuously, restarting the connection to the server after certain events.
2. The client may print out some state information between sessions. Please ignore these logs, as they do not indicate the start of a new session.