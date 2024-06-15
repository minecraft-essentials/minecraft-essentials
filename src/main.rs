use clap::{Args, Parser, Subcommand};
use minecraft_essentials::{AuthType, AuthenticationBuilder};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Version {},
    /// Oauth Check command.
    Oauth(OauthArgs),
    /// DeviceCode Check command.
    DeviceCode(DeviceCodeArgs),
    // / Custom Launch Check command
    // CustomLaunch(CustomLaunchArgs),
}

#[derive(Args)]
struct OauthArgs {
    client_id: String,
    client_secret: String,
    port: Option<u16>,
    bedrockrelm: Option<bool>,
}

#[derive(Args)]
struct CustomLaunchArgs {
    token: String,
    uuid: String,
    optional_args: String,
    java_exe: String,
    jrepath: Option<PathBuf>,
    offline: Option<bool>,
}

#[derive(Args)]
struct DeviceCodeArgs {
    client_id: String,
    bedrockrelm: bool,
}

pub(crate) const EXPERIMENTAL_MESSAGE: &str =
    "\x1b[33mNOTICE: You are using an experimental feature.\x1b[0m";

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Oauth(oauth_args) => handle_oauth(oauth_args).await,
        Commands::DeviceCode(device_code_args) => handle_device_code(device_code_args).await,
        Commands::Version {} => println!("{}", env!("CARGO_PKG_VERSION")),
        // Commands::CustomLaunch(handle_custom_launch_args) => {
        //     handle_custom_launch(handle_custom_launch_args).await
        // }
    }
}

async fn handle_oauth(oauth_args: &OauthArgs) {
    let mut auth_builder = AuthenticationBuilder::builder();
    auth_builder
        .of_type(AuthType::Oauth)
        .await
        .client_id(&oauth_args.client_id)
        .client_secret(&oauth_args.client_secret)
        .bedrockrel(oauth_args.bedrockrelm)
        .port(oauth_args.port);

    println!("{:?}", auth_builder.get_info());

    let auth_info = auth_builder.launch().await.unwrap();

    println!("{:?}", auth_info)
}

async fn handle_device_code(device_code_args: &DeviceCodeArgs) {
    let mut auth_builder = AuthenticationBuilder::builder();
    auth_builder
        .of_type(AuthType::DeviceCode)
        .await
        .client_id(&device_code_args.client_id)
        .bedrockrel(Some(device_code_args.bedrockrelm));

    println!("{:?}", auth_builder.get_info());

    println!("{:?}", auth_builder.launch().await);
}

// async fn handle_custom_launch(handle_custom_launch_args: &CustomLaunchArgs) {
//     let mut args = Vec::new();

//     args.push(format!("--token{}", handle_custom_launch_args.token));
//     args.push(format!("--uuid{}", handle_custom_launch_args.uuid));

//     if !handle_custom_launch_args.optional_args.is_empty() {
//         args.push(handle_custom_launch_args.optional_args.clone())
//     }

//     let launch = Launch::new(
//         args,
//         handle_custom_launch_args.java_exe.clone(),
//         handle_custom_launch_args.jrepath.clone(),
//         handle_custom_launch_args.offline,
//     )
//     .expect("Expected Launch");

//     let launch_info = launch.info();

//     println!("Launching with: {:?}", launch_info);

//     let _ = launch.launch_jre();
// }
