## Start Your Web3 Journey  

This is a **Stellar Soroban learning repository using Rust**.

We will begin by setting up Rust and covering some basic Rust concepts.

### Install Rust

To get Rust, please visit [rust-lang.org](https://www.rust-lang.org).

❗❗❗You need to update the path after installation of rust. read the message after installation do not close your terminal after initial installation command completed. Read on screen instructions. ❗❗❗

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
[`stellarCLI/soroban-hello-world-contracts-hello_world/src` directory](https://github.com/onuralpArsln/stellarSoroban/tree/main/stellarCLI/soroban-hello-world/contracts/hello_world/src) you can proceed to compile this project.


### Run the Test

On your terminal execute 

```
cargo test
```
 to run your unit test expected output is a success message. You need to be on your `soroban-hello-world` directory.

 Note that for your first time it needs to compile many files so you will be seeing lots of compiling output during first test.

### Build the Contract

To build this project run

 ```
stellar contract build
```

one expected error in this step is 
>can't find crate for 'core'

which causes due to lack of installation wasm32 target during setup. Can be solved by:
```
rustup target add wasm32-unknown-unknown
```

If problem persists after this refer to following section.

To check targets you have installed on your device you can run 

```
rustup target list --installed
```

and if you want to remove one  use 
```
rustup target remove <target_name>

```

The `stellar contract build`  command is a short hand for `cargo build --target wasm32-unknown-unknown --release`. It makes cargo build to target wasm32-unknown-unknown and makes its profile to release.
A `.wasm` file should appear on `target/wasm32-unknown-unknown/release/hello_world.wasm`

The extension `.wasm` stands for **WebAssembly** binary file.


---
#### 🔔 If "error[E0463]: can't find crate for `core`" persists

There are few methods you can take to solve your problem. After each step try `stellar contract build` to see if problem persists. 

1- clear cargo and update
A solid for approach for many problem is to update things. Although i rarely see that an update solve problems it is often the simplest thing you can do.

```sh
cargo clean 
rustup update
cargo update
```

2- remove toolchain and target then reinstall them

it is a simple "restart to solve"

```sh
rustup target remove wasm32-unknown-unknown
rustup toolchain uninstall stable
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
```

3- Reinstall rust

It can solve many problems you might not aware of. If somehow your installation gets corrupted you might need to perform this step. This solved the problem in my case.

❗❗❗You need to update the path after installation of rust. read the message after installation do not "enter enter skip" it❗❗❗

```sh
rustup self uninstall
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation completed, you need to update the path. In my case i performed this by following command since i use  **fish** the friendly interactive shell i need to use this command. But if you use bash zsh or anything other, after installation you can  see which command you should use on your screen just copy the command related to your shell. (time of my install there were two for fish and for others)

```sh
source "$HOME/.cargo/env.fish"
```

then update to be safe

```sh
rustup update
```

---


### Optimizing Builds

It is important to reduce file size when working with anything web related. Smaller file means less download time making process go faster.

So if you do not have the `opt` feature on your stellar-cli you need to download it. To check a feature in your stellar-cli you can run 

```
stellar <feature_name> --help

```

in our case run 

```
stellar contract optimize  --help

```

This is a rule of thumb you can check with --help to see if your command chain is installed.
 

if there is no command as you need to run  

```
cargo install --locked stellar-cli --features opt
```
It might say `Ignored package `stellar-cli v21.5.0` is already installed, use --force to override` this means you already have it installed do not `--force`  it.

If you have "optimize" feature installed 
run `ls` and look for the `target` folder.
Be sure that you are on `soroban-hello-world` file as your current working directory.

Then optimize the release 

```sh
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/hello_world.wasm
```

---

## Deploy Time 

In blockchain world testnet is a mockup part. You can think it as a "playground" or "sandbox" 
versions of projects allowing your work to be tested without real world risks.

To deploy we first need to prepare our CLI for deployment.
much of next steps are from -> [`Setup Stellar Configuring the CLI for Testnet `](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup#configuring-the-cli-for-testnet)

I am writing for unix, if you are on windows complete the setup process from the link given above.

Lets setup for testnet, that is not a deployment server it is for testing purposes.

```sh
stellar network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
  ```

This creates `~/.config/soroban/network/testnet.toml` and allows your cli commands to be used for testnet. `--network testnet` can be used.

For testnet you need and identity.(for any smart contract you need test)

We will generate a key named alice, but you can go creative with names. I purposefully use same name on tutorial because this repo aims to help anyone who partly looks for tutorial and haves some problems with it. 

```sh
stellar keys generate --global alice --network testnet
```
after a brief wait command completes

to check you have a dress now

```sh
stellar keys address alice
```

you can try different names then alice and you will see that there is no identity.

Now we are ready to deploy with alice address, command below will yield a id, record it. It is end line copy it. 
And Record it.
This is a public value like name of a youtube video.

Deployment Command 

```sh
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source alice \
  --network testnet
```

my id for this is `CDJW3Z6KNDIEFAXIRAA3CFLOZQYUTON2DZPDLD46U4CDI3W6CFSL75XR`

also you will have a link like `https://stellar.expert/explorer/testnet/contract/CDJW3Z6KNDIEFAXIRAA3CFLOZQYUTON2DZPDLD46U4CDI3W6CFSL75XR` basically it is `https://stellar.expert/explorer/testnet/contract/` +`your testnet id`

So you can check your details over that link

---

## Interact with your code  

We will make RPC calls with CLI

What is RPC? 
Remote Procedure Call is a software communication protocol allowing requesting service from a program located on different machine or network without knowing whats going on in background.

Isn't it what APIs do? 
API is the framework, the general structure. RPC is how they "talk" within that framework.

Lets invoke our contract.

Remember you need to change you id if you want to reach to your own contract. if you copy paste this command you will be invoking my contract. Also you will return a 

```sh
stellar contract invoke \
  --id CDJW3Z6KNDIEFAXIRAA3CFLOZQYUTON2DZPDLD46U4CDI3W6CFSL75XR \
  --source alice \
  --network testnet \
  -- \
  hello \
  --to RPC
  ```


where does the `hello` comes from?
look at src/lib.rs you will see 

```rust
impl HelloContract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}
```

It defined the public function hello, we simply called that.


----------


## How to store data 

With smart contracts you can store data.

We will do an increment example where we count up and store value.

In a sinle project you can init many contracts.


```sh
stellar contract init ./ --with-example increment
```

this command will initialize a new contract called increment.

your end file structure will be 


```sh
└── contracts
    ├── increment
        ├── Cargo.lock
        ├── Cargo.toml
        └── src
            ├── lib.rs
            └── test.rs
```
Now we have an example setup in `contracts/increment`.

Look at the lib.rs but we will explain details here about the code

exclude rust std and include basics 

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};
```

###  Contract Data keys

```rust
const COUNTER: Symbol = symbol_short!("COUNTER");
```

You create a key for data you want to store and 
you can call this key to learn data later

The symbol_short!() macro is a convenient way to 
pre-compute short symbols up to 9 characters in 
length at compile time using Symbol::short.


To use count 

```rust
// from contracts/increment/src/lib.rs
let mut count: u32 = env
    .storage()
    .instance()
    .get(&COUNTER)
    .unwrap_or(0); // If no value set, assume 0.
```

Rust part of this  `let mut count: u32`


The Env.storage() function is used to access and update contract data.
The executing contract is the only contract that can query or modify contract 
data that it has stored. The data stored is viewable on ledger anywhere the ledger
is viewable, but contracts executing within the Soroban environment are restricted to their own data.

The get() function gets the current value associated with the counter key.

If no value is currently stored, the value given to unwrap_or(...) is returned instead.


```rust
env.storage()
    .instance()
    .set(&COUNTER, &count);
```

With set you define a new number

```rust
env.storage().instance().extend_ttl(100, 100);
```

All contract data has a Time To Live (TTL), measured in ledgers, that must be periodically extended. If an entry's TTL is not periodically extended, the entry will eventually become "archived." You can learn more about this in the State Archival document.
 Every time the counter is incremented, this storage's TTL gets extended by 100 ledgers, or about 500 seconds.


## Build it again

remeber to be on inside stellar-hello-world and be able to see contracts when running `ls`
```sh
stellar contract build
```

lets see your wasm builds with 

```sh
ls target/wasm32-unknown-unknown/release/*.wasm
```
## Deploy time 

To deploy see how i change the name of release file 

```sh
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroban_increment_contract.wasm \
  --source alice \
  --network testnet
```

It yielded me a new id and i recorded it.

## See some increment

```sh

stellar contract invoke \
  --id CAFGG6EPIGYAEJ7DZWLOYRVQ75SPRH4MUOCKFRQMZMGVYBICDLLSJRLQ \
  --source alice \
  --network testnet \
  -- \
  increment

```

Try repeating this command.


# A Project With Frontend

Up to this point we discussed basics on terminal lest move on with some shiny frontend.

to start with front end we will move to completely different folder. So go back to root of this project and start a rust project

be sure that you are on root of your project before moving on. Or else our structure will be complicated to follow.

Now i wil create a new project named `frontend_soroban`. But do not just make a folder we are trying to create boiler plate on auto pilot so we have some tricks.

So start a new rust project and move into it.

```sh
cargo new --lib frontend_soroban
cd frontend_soroban
```

for our new project we need to fill up our cargo.toml a bit so add those to

```rust
// frontend_soroban/Cargo.toml

[package]
name = "frontend_soroban"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = "0.9.2"

[dev-dependencies]
soroban-sdk = { version = "0.9.2", features = ["testutils"] }

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true

```


Now lets work on our lib.rs with 

```rust
// frontend_soroban/src/lib.rs 
#![no_std]
use soroban_sdk::{contractimpl, symbol, Env, Symbol};

pub struct FrontendContract;

#[contractimpl]
impl FrontendContract {
    pub fn greet(env: Env, name: Symbol) -> Symbol {
        let prefix = Symbol::new(&env, "Welcome_");
        Symbol::join(&env, &[prefix, name])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test() {
        let env = Env::default();
        let contract_id = env.register_contract(None, FrontendContract);
        let client = FrontendContractClient::new(&env, &contract_id);

        let name = Symbol::new(&env, "Developer");
        let result = client.greet(&name);
        assert_eq!(result, Symbol::new(&env, "Welcome_Developer"));
    }
}
```

before moving on to our front end lets build this new contract to testnet.

build with 

```sh
soroban contract build
```
