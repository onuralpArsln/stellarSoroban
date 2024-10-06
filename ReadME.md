## This is a stelart soroban learning repo using rust.


we will start with getting rust and using rust basics.

to get rust please visit -> rust-lang.org

to check your rust installation you can use 

rustc -V

rustc --version

cargo --version

cargo -V

to see if those commands return a value, generally you can use -V or --version to check installations. Beware that you might need to restart your terminal to ensure that freshly installed commands are recognized by your system. 

then try rustlings course from ->  https://github.com/rust-lang/rustlings/

to use rustlings course  

+ cargo install rustlings
+ rustlings init
+ cd rustlings/
+ rustlings

you can also use rustrover for a comfortable rust development, at the time of this repo being written rust rover is a free for non-commercial use ide for rust. You can definitely go with vsc zed vim sublime text or any other editor you might enjoy.

you should learn about cargo which is the package manager for rust 
you can use rustc to compile but cargo build makes it better for larger projects  with dependeices



to get started with stellar you can visit ->  https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup
to installer stellar cli

## First Contract
______________________________
for first project we will follow -> https://developers.stellar.org/docs/build/smart-contracts/getting-started/hello-world

This simply creates a contract and explains the structure
On our repo it is within stellarCLI/soroban-hello-world


if you already installad stellar cli you can now generate a stealer cli project 

use stellar contract init soroban-hello-world to generate your first contract 

cargo.toml  is a rust worksapce that allows multiple smart contract inside one project

rust workspace is a cargo feature that allows management of multiple packages inside single project


