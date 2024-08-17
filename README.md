# Minecraft-Essentials

The official rust/cargo package that provides essential functionality for Minecraft client launchers.

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
minecraft-essentials = "0.2.12"
```

## Usage

### Authentication

This is an example of how to use the authentication builder.
For this example we will be using the Oauth method.
```rust, ignore
use minecraft_essentials::{AuthenticationBuilder, AuthType};
use std::env;
use tokio::main;

#[tokio::main]
async fn main() {
  let client_id = "ClientID here";
  let client_secret = "Client_Secret here"; // Recommended to use env to store secrets
  let mut builder = AuthenticationBuilder::builder();
  builder
      .of_type(AuthType::Oauth)
      .client_id(&client_id)
      .client_secret(&client_secret) // Only Required for ouath
      .port(Some(8000)); // Optional for ouath but defaults to port 8000
  println!("Info: {:?}", builder.get_info().await); // users info 
  println!("Authentifcation Final Info: {:?}", builder.launch().await.unwrap()); // for your launcher.
}
```

### Launching
```rust, ignore
use minecraft_essentials::LaunchBuilder;
use std::path::PathBuf;
  let args = ["--argexample 123"] 
  let mut builder = LaunchBuilder::init();
  builder
      .args(args)
      .java(Some(PathBuf::from("C:\\Program Files\\Java\\jdk-17.0.1\\bin\\java.exe"))) // Custom Java Path for custom java
      .client(Some(PathBuf::from("C:\\Users\\User\\Desktop\\Client.jar"))) // Minecraft Client Path for custom client
      .mods(Some(vec![PathBuf::from("C:\\Users\\User\\Desktop\\Mod1.jar"), PathBuf::from("C:\\Users\\User\\Desktop\\Mod2.jar")])) // Custom Mods Path for custom mods (Optional)
      .launch(None).await // Launches Minecraft
```

## Contributing

Interested in contributing to this project? Check out [Contributing](./contributing.md).

## Licensing

This library is licensed under the [BSD 3.0 License](./LICENSE).

## Credits

- [trippleawap](https://github.com/trippleawap) for providing a Minecraft Authentication Sample for Minecraft Implementation.


<!-- Links -->

[Docs]: https://docs.rs/minecraft-Essentials
[Node]: https://github.com/minecraft-essentials/Node
[Python]: https://github.com/minecraft-essentials/Python
[Roadmap]: https://github.com/orgs/minecraft-essentials/projects/1