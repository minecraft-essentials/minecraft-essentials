
# Minecraft-Essentials

A Package that gives all Minecraft client launchers essentials.

## Features

- Essential - functionality for Minecraft Client Launchers
- Simplifies - Minecraft Client Launcher Building.
- Fast - performs better than other frameworks in authentification and launching
- Safe - Forbids Unsafe Code `#![forbid(unsafe_code)]`
- Beginner Friendly - Full [documentation](https://docs.rs/minecraft-Essentials) and examples/template avalible on github.



## Where is the Other Packages???

Some of the packages had to be split up for reasons they are located here:
- [NPM (Node)](https://github.com/minecraft-essentials/npm)
- [PyPi (Coming soon!)](https://github.com/minecraft-essentials/pypi)

---

## Installation

Prerequisites: 
- Rust



## Usage:

### Oauth BearToken:
```rust
use minecraft_essentials::*;


let client_id = "";
let client_secret = "";
let port = None;

let bedrockrel = false;
let auth = Oauth::new(client_id, port);

println!("{}", auth.url());

let auth_info = auth.launch(bedrockrel, client_secret).await;

println!("{:?}", auth_info)
```


### Oauth Bedrock Relm:
```rust
use minecraft_essentials::*;


let client_id = "";
let client_secret = "";

let port = None;
let bedrockrel = true;

let auth = Oauth::new(client_id, port);
println!("{}", auth.url());

let auth_info = auth.launch(bedrockrel, client_secret).await;

println!("{:?}", auth_info)
```

---


### Device_Code
```rust, ignore
use minecraft_essentials::*;
let client_id = "111231209837123098712";
let code = device_code::new(client_id);
println!("Stuff Here: {}", code.prelaunch());

let code_info = code.launch().await?;
```


**More usages coming soon.**


# Licensing

This library is licensed under the [BSD 3.0 Licence](./LICENSE)
