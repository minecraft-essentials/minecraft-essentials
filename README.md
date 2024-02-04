<!-- 
Copyright (C) 2024 Mincraft-essnetials 

* This program is free software: you can redistribute it and/or modify it
* under the terms of the GNU Affero General Public License as published by
* the Free Software Foundation, either version 3 of the License, or (at your
* option) any later version.
* 
* This program is distributed in the hope that it will be useful, but WITHOUT
* ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
* FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public
* License for more details.
* 
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
 -->


# Minecraft-Essentials

A Package that gives all Minecraft client launchers essentials.

## Features

- Essential - functionality for Minecraft Client Launchers
- Simplifies - Minecraft Client Launcher Building.
- Fast - performs better than other frameworks in authentification and launching
- Safe - Forbids Unsafe Code `#![forbid(unsafe_code)]`
- Beginner Friendly - Full [documentation](https://docs.rs/minecraft-Essentials) and examples/template avalible on github.



## Notices

**This package/library is not a virus/stealer and all of the code is easily shown. We take extra procautions for viruses and stealers in the code.**

**Some packages/libraries had to be split up into separate repositories. The list is provided below. This is future versions will be combined with packages.**

- [NPM (Node)](https://github.com/minecraft-essentials/npm)

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

This library is licensed under the [Affero-GPL-3.0 Licence](./LICENSE)
With the licence above the header is provided [here](./HEADER)
For the templates see the [README](./templates/README.md)
