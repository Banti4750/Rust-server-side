# Simple-Log

A simple web server built with Rust and the Nickel framework that responds with "Hello dear world!" to all GET requests.

## Description

Simple-Log is a minimalist web application that demonstrates the basic usage of the Nickel web framework for Rust. It creates a web server that listens on port 6767 and responds to all GET requests with a friendly greeting.

## Features

- Simple HTTP server implementation using Nickel
- Asynchronous runtime with Tokio
- Responds to all GET requests with a greeting message

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

## Installation

1. Clone the repository or download the source code
2. Navigate to the project directory

```bash
cd simple-log
```

3. Build the project

```bash
cargo build
```

## Usage

Run the application with:

```bash
cargo run
```

Once the server is running, you can access it by navigating to [http://127.0.0.1:6767](http://127.0.0.1:6767) in your web browser or using tools like curl:

```bash
curl http://127.0.0.1:6767
```

You should receive the response: "Hello dear world!"

## Dependencies

- [Nickel](https://github.com/nickel-org/nickel.rs) - Web application framework for Rust
- [Tokio](https://tokio.rs/) - Asynchronous runtime for Rust

## License

This project is open source and available under the MIT License.

## Author

Joel McCracken <mccracken.joel@gmail.com>