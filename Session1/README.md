# Installing Rust

## Try Rust Without Installing

You can experiment with Rust online using the Rust Playground without installing anything on your computer.

## Rustup: The Rust Installer and Version Management Tool

The primary method for installing Rust is via Rustup, a Rust installer and version management tool.

### Installation Instructions for macOS, Linux, or other Unix-like OS

To download Rustup and install Rust, open your terminal and execute the following command, then follow the on-screen instructions:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
## Check for Updates

Rust updates frequently. If you installed Rustup some time ago, your Rust version might be outdated. Update Rust by running:
```bash
rustup update
```

Learn More About Installation
Cargo: The Rust Build Tool and Package Manager

Upon installing Rustup, you will also receive the latest stable version of Cargo, the Rust build tool and package manager. Cargo can:
```
Build your project with cargo build
Run your project with cargo run
Test your project with cargo test
Build documentation with cargo doc
Publish a library to crates.io with cargo publish
```

Testing Installation

Verify that Rust and Cargo are correctly installed by running:

bash

cargo --version

Read The Cargo Book
Other Tools

Rust support is available in many code editors:

    VS Code
    Sublime Text
    RustRover
    Eclipse
    Vim
    Emacs
    Visual Studio

Generating a New Project

To start a new Rust project:

bash

cargo new hello-rust

This command generates a new directory called hello-rust with the following structure:

bash

hello-rust/
|- Cargo.toml     # The manifest file for your project, contains metadata and dependencies.
|- src/
   |- main.rs     # The main application code.

cargo new creates a basic "Hello, world!" project.
Running Your Project

Navigate into your new project directory and run:

bash

cargo run

You should see the following output in your terminal, indicating your program has run successfully:

scss

   Compiling hello-rust v0.1.0 (/path/to/hello-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 1.34s
     Running `target/debug/hello-rust`
Hello, world!