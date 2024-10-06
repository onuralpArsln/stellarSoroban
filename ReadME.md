## Start Your Web3 Journey  

This is a **Stellar Soroban learning repository using Rust**.

We will begin by setting up Rust and covering some basic Rust concepts.

### Install Rust

To get Rust, please visit [rust-lang.org](https://www.rust-lang.org).

After installing Rust, you can verify your installation by running:

```sh
rustc -V
rustc --version
cargo --version
cargo -V
```

These commands should return version information for `rustc` (the Rust compiler) and `cargo` (Rust's package manager). You may need to restart your terminal to ensure the newly installed commands are recognized by your system.

### Learn Rust Basics

For this project you do not required to know intensive amount of rust. But it is nice to know. you can quickly realize that it has a `C`/`C++` like syntax and feel. You can go for the `thisRepo/rustBasics` within this repo and then `thisRepo/cargoTime` to learn enough rust and cargo for this project or you can delve deeper with the Rustlings course.
We recommend going on with the Rustlings course to learn rust in a robust way, which you can find here: [Rustlings GitHub Repository](https://github.com/rust-lang/rustlings/).

To use the Rustlings course, run:

```sh
cargo install rustlings
rustlings init
cd rustlings/
rustlings
```

### Choosing an IDE

For a comfortable Rust development experience, you can use **RustRover**, which is currently free for non-commercial use. Alternatively, you can use **VS Code**, **Zed**, **Vim**, **Sublime Text**, or any other editor you prefer.

### Learn About Cargo

**Cargo** is Rust's package manager and build system. Although you can compile Rust code using `rustc`, using `cargo build` is more convenient for larger projects with dependencies. In `cargoTime` of this repo you can find simple introduction to cargo. Since it is a package manager if you used any package manager before, it is more or less similiar user experince. 

### Getting Started with Stellar

To get started with Stellar and set up the environment, visit: [Stellar Developers - Getting Started](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup). This page also includes instructions on how to install the Stellar CLI.

---

## First Contract

For your first project, we will follow the tutorial here: [Stellar Hello World Contract](https://developers.stellar.org/docs/build/smart-contracts/getting-started/hello-world).

This part is a my starter friendlier version of it explaining small details in simpler terms and adding comments inside the code snippets to make easier to read through for starters.

This tutorial helps you create a simple contract and explains the structure. In this repository, you can find it under `thisRepo/stellarCLI/soroban-hello-world`.

### Generate Your First Contract

If you have already installed the Stellar CLI, you can generate a Stellar CLI project using:

```sh
stellar contract init soroban-hello-world
```

### Understanding `Cargo.toml` and Rust Workspaces

`Cargo.toml` is a configuration file used in Rust projects. It allows you to manage dependencies and define project metadata.

A **Rust workspace** is a feature of Cargo that allows you to manage multiple packages within a single project. This is useful when building larger applications with multiple smart contracts or libraries.

After reading trough the main.rs and lib.rs of this first project from 
[`.stellarCLI/soroban-hello-world/contracts/hello_world/src`](.stellarCLI/soroban-hello-world/contracts/hello_world/src) you can proceed to compile this project.


### Run the Test

On your terminal execute `cargo test` to run your unit test expected output is a success message.

### Build the Contract

To build this project run `stellar contract build`

one expected error in this step is 
>can't find crate for 'core'

which causes due to lack of installation wasm32 target during setup. Can be solved by 
```
rustup target add wasm32-unknown-unknown
```
---
