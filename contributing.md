# Contributing to the Main(Rust) Version


## Before you start:
- You Read the [conventional commit specification](https://www.conventionalcommits.org/en/v1.0.0/),
- You have basic knowledge of how [rust / A library works](https://doc.rust-lang.org/cargo/index.html) &
- And At least done a [contributed at least once](https://github.com/firstcontributions/first-contributions).


## Prerequisites: 
- Rust,
- [Azure/EntraId App with Client Id And Secret](https://entra.microsoft.com/) &
- [Access to the Bearer Token API.](https://help.minecraft.net/hc/en-us/articles/16254801392141p)



**Note: The bearer Token API can take month/years to get.**



## Getting Started

First you want to get started by creating a `.env` and copy and paste the `.env.example` to the `.env`.


The Client_ID is the client id from your [Azure/EntraId App](https://entra.microsoft.com/)
The Client_Secret is the client Secret from your [Azure/EntraId App](https://entra.microsoft.com/)

Note in your [Azure/EntraId App](https://entra.microsoft.com/) you need to set the redirect url to `http://localhost:PORT` The Port is `PORT`. Useally you want to set it to the port that you specifyed but if you haven't set it yet the deafult port is 8000 or `http://localhost:8000`. 



### Testing/Benchmarks/Running 

In `lib.rs` there is built in tests for testing the library using the `cargo test`. 

If you want to just try out things you can play around with stuff in `main.rs` then to run it just do a simple `cargo run` to run the main.rs for playaround stuff and testing.

To benchmark you must write a test in `lib.rs` based on [cargo-bench](https://doc.rust-lang.org/cargo/commands/cargo-bench.html) and it will tell you in seconds how long your change takes.




## Submiting a PR

We expect you to follow the PR Template Format and use the check list, We put that there to help you create a PR. 