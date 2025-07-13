# localtrace

[<img alt="github" src="https://img.shields.io/badge/github-ysuzuki19/localtrace-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/ysuzuki19/localtrace)
[<img alt="crates.io" src="https://img.shields.io/crates/v/localtrace.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/localtrace)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-localtrace-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/localtrace)

A lightweight Rust library for enhanced error handling with local backtrace information.

## Overview

`localtrace` provides a simple error handling mechanism that captures and displays backtrace information filtered to show only your local project files. This makes debugging much easier by focusing on your code rather than system libraries.

## Features

- 🎯 **Local-only backtraces**: Only shows stack frames from your project directory
- 🚀 **Zero-cost in release builds**: Backtrace collection is disabled in release mode
- 🛠️ **Simple API**: Drop-in replacement for standard `Result<T, E>`
- 🔧 **Convenient macros**: Easy error creation with the `trace!` macro

## Installation

Run the following command to add `localtrace` to your project:

```bash
cargo add localtrace
```

## Quick Start

```rust
use localtrace::{Result, trace, catch_with_trace};

fn might_fail() -> Result<String> {
    let content = std::fs::read_to_string("config.txt")?;

    if content.is_empty() {
        return trace!("Configuration file is empty");
    }

    Ok(content)
}

fn main() {
    catch_with_trace(|| {
        let config = might_fail()?;
        println!("Config: {}", config);
        Ok(())
    });
}
```

```bash
$ cargo run
message: No such file or directory (os error 2)
- /path/to/project/localtrace/example/src/readme_quickstart.rs:4
- /path/to/project/localtrace/example/src/readme_quickstart.rs:15
- /path/to/project/localtrace/example/src/readme_quickstart.rs:14
```

The error is triggered at the line where `read_to_string` is called. The backtrace shows the sequence of function calls, specifically pointing to the lines in `might_fail()` and `catch_with_trace()` where the error was propagated.

## API Reference

### Types

#### `Result<T>`

A type alias for `std::result::Result<T, Error>` that provides enhanced error information.

#### `Error`

The main error type that captures:

- Error message
- Local backtrace (debug builds only)

### Functions

#### `catch_with_trace<F>(f: F)`

Executes a function and prints any errors to stderr.

```rust
localtrace::catch_with_trace(|| {
    // Your code here
    Ok(())
});
```

### Macros

#### `trace!(message)`

Creates an error with the given message.

```rust
use localtrace::{Result, trace};

// trace! can receive a simple message
fn simple_message() -> Result<()> {
    trace!("Something went wrong")
}

// trace! can receive a formatted message
fn formatted_message(id: u32, reason: &str) -> Result<()> {
  trace!("Failed to process item {}: {}", id, reason)
}
```

## How It Works

In debug builds, `localtrace` captures a full backtrace when an error occurs, then filters it to show only stack frames from your project directory (determined by the `CARGO_MANIFEST_DIR` environment variable). This gives you precise error location information without the out of crate calling.

In release builds, backtrace collection is completely disabled for zero runtime overhead.

## License

This project is licensed under the Mozilla Public License 2.0 (MPL-2.0).

Example code is licensed under the MIT License.
