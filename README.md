# Minecraft-Essentials

The offical cargo package that provides essential functionality for Minecraft client launchers.

## Features

- **Essential**: Offers core functionality for Minecraft Client Launchers.
- **Simplifies**: Streamlines the process of building Minecraft Client Launchers.
- **Fast**: Delivers superior performance in authentication and launching.
- **Safe**: Ensures safety by forbidding unsafe code.
- **Beginner Friendly**: Comprehensive [documentation][Docs] and examples/templates available on GitHub.

## Package Locations

- [Node][NPMRepo]

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

### OAuth Custom Authentifcation | OAuth2.0

This example demonstrates how to use the OAuth authentication method provided by `minecraft-essentials`, `oauth` feature.

```rust, ignore
use minecraft_essentials::*;

// Initialize OAuth authentication with your client ID and an optional custom port
let client_id = "";
let client_secret = "";

let port = None; // Optional: Set a custom port
let bedrockrel = false; // Optional: Use this flag to only get the XTS token

// Initialize the OAuth authentication object
let auth = Oauth::new(client_id, port);

// Print the URL needed for authentication
println!("URL: {}", auth.url());

// Launch the authentication process
 let auth_info = auth.launch(bedrockrel, client_secret).await;

// Print the authentication information
println!("{:?}", auth_info)
```




### Device Code Custom Authentication | DeviceCode

This example demonstrates how to use the Device Code authentication method provided by `minecraft-essentials`, `devicecode` feature.

```rust, ignore 
use minecraft_essentials::*;

// Initialize Device Code authentication with your client ID 
let client_id = "111231209837123098712";

// Create a new device code instance 
let code = device_code::new(client_id);

// Print the device code information 
println!("Stuff Here: {}", code.preinfo());

// Launch the authentication process 
let code_info = code.launch().await?;
```



## Contributing

Interested in contributing to this project? Check out [Contributing](./contributing.md).

## Licensing

This library is licensed under the [BSD 3.0 License](./LICENSE).

## Credits

- [trippleawap](https://github.com/trippleawap) for providing a Minecraft Authentication Sample for Minecraft Implementation.


<!-- Links -->

[Docs]: https://docs.rs/minecraft-Essentials
[Node]: https://github.com/minecraft-essentials/node
