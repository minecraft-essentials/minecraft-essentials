# Minecraft-Essentials
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fminecraft-essentials%2Fminecraft-essentials.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fminecraft-essentials%2Fminecraft-essentials?ref=badge_shield)


The offical rust/cargo package that provides essential functionality for Minecraft client launchers.

## Features

- **Essential**: Offers core functionality for Minecraft Client Launchers.
- **Simplifies**: Streamlines the process of building Minecraft Client Launchers.
- **Fast**: Delivers superior performance in authentication and launching.
- **Safe**: Ensures safety by forbidding unsafe code.
- **Beginner Friendly**: Comprehensive [documentation][Docs] and examples/templates available on GitHub.

## Package Versions
If your looking to use a package other than rust/cargo you might want to have a look at:

- [Node Version for JS/TS][Node]
- [Python Version][Python]

## Installation

Add `minecraft-essentials` to your project:

```sh
cargo add minecraft-essentials
```

**OR**

Add the following to your `Cargo.toml`:

```toml
[dependencies]
minecraft-essentials = "0.2.9"
```

## Usage

### Authentifcation
#### OAuth Custom Authentifcation | OAuth2.0

This example demonstrates how to use the OAuth authentication method provided by `minecraft-essentials`, `oauth` feature.

```rust
use minecraft_essentials::*;

async fn Oauth(client_id: &str, client_secret: &str, port: Option<u16>, bedrockrel: bool) {
// Initialize the OAuth authentication object
let auth = Oauth::new(client_id, port);

// Print the URL needed for authentication
println!("URL: {}", auth.url());

// Launch the authentication process
 let auth_info = auth.launch(bedrockrel, client_secret).await;

// Print the authentication information
println!("{:?}", auth_info)
}

fn main() {
    Oauth("CLientID", "ClientSecret", None, false);
}
```

#### Device Code Custom Authentication | DeviceCode

> [!WARNING]
> This is still work in progress **so it may change**.


This example demonstrates how to use the Device Code authentication method provided by `minecraft-essentials`, `devicecode` feature.

```rust, ignore
use minecraft_essentials::*;


async fn deviceCode(client_id: &str) {
  // Create a new device code instance 
  let code = DeviceCode::new(client_id).expect("Expected Code");

  // Print the device code information 
  println!("Stuff Here: {}", code.preinfo());

  // Launch the authentication process 
  let code_info = code.launch().await?;
}

fn main() {
    // Initialize Device Code authentication with your client ID 
    deviceCode("111231209837123098712");
}
```

#### Acutal Minecraft Authentfication

> [!CAUTION]
> This is currently in the [roadmap][Roadmap] for 0.2.12-14 currently it's not avalible.


### Launching

#### Custom Launching 
```rust
use minecraft_essentials::Launch;
use std::path::Path;

let args = vec!["--uuid:LauncherUUID".to_string(), "--token:Beartoken".to_string()];
let jre_path = Path::new("/path/to/jre").to_path_buf();
let java_exe = "/your/java/path";

// Init the instance of launch
let launch = Launch::new(args, java_exe.to_string(), Some(jre_path.clone()), Some(false)).expect("Expected Launch");

// Grab the info to verify that your doing everything currect.
let launch_info = launch.info();
println!("Launching with: {:?}", launch_info);

let _ = launch.launch_jre();
```

## Contributing

Interested in contributing to this project? Check out [Contributing](./contributing.md).

## Licensing

This library is licensed under the [BSD 3.0 License](./LICENSE).


[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fminecraft-essentials%2Fminecraft-essentials.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fminecraft-essentials%2Fminecraft-essentials?ref=badge_large)

## Credits

- [trippleawap](https://github.com/trippleawap) for providing a Minecraft Authentication Sample for Minecraft Implementation.


<!-- Links -->

[Docs]: https://docs.rs/minecraft-Essentials
[Node]: https://github.com/minecraft-essentials/Node
[Python]: https://github.com/minecraft-essentials/Python
[Roadmap]: https://github.com/orgs/minecraft-essentials/projects/1