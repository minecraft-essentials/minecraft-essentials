use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use minecraft_essentials::{AuthType, AuthenticationBuilder, LaunchBuilder};

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
    /// Minecraft Launching Check command.
    Launch(LaucnhArgs),
}

#[derive(Args)]
struct OauthArgs {
    client_id: String,
    client_secret: String,
    port: Option<u16>,
    bedrockrelm: Option<bool>,
}

#[derive(Args)]
struct LaucnhArgs {
    // Java Args
    min_memory: usize,
    max_memory: Option<usize>,
    launcher_name: Option<String>,
    launcher_version: Option<String>,
    jre: Option<String>,
    class_path: Option<String>,

    // Game Args
    client_id: Option<String>,
    username: Option<String>,
    version: Option<String>,
    uuid: Option<String>,
    game_directory: Option<PathBuf>,
    width: Option<usize>,
    height: Option<usize>,
    access_token: Option<String>,

    // Quick Play Args
    quick_play_singleplayer: Option<String>,
    quick_play_multiplayer: Option<String>,
}

#[derive(Args)]
struct DeviceCodeArgs {
    client_id: String,
    bedrockrelm: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Oauth(oauth_args) => handle_oauth(oauth_args).await,
        Commands::DeviceCode(device_code_args) => handle_device_code(device_code_args).await,
        Commands::Version {} => println!("{}", env!("CARGO_PKG_VERSION")),
        // TODO: HANDLE LAUNCH
        Commands::Launch(arg) => todo!(),
    }
}

async fn handle_oauth(oauth_args: &OauthArgs) {
    let mut auth_builder = AuthenticationBuilder::builder();
    auth_builder
        .of_type(AuthType::Oauth)
        .client_id(&oauth_args.client_id)
        .client_secret(&oauth_args.client_secret)
        .bedrockrel(oauth_args.bedrockrelm)
        .port(oauth_args.port);

    println!("{:?}", auth_builder.get_info().await);

    let auth_info = auth_builder.launch().await.unwrap();

    println!("{:?}", auth_info)
}

async fn handle_device_code(device_code_args: &DeviceCodeArgs) {
    let mut auth_builder = AuthenticationBuilder::builder();
    auth_builder
        .of_type(AuthType::DeviceCode)
        .client_id(&device_code_args.client_id)
        .bedrockrel(Some(device_code_args.bedrockrelm));

    println!("{:?}", auth_builder.get_info().await);

    println!("{:?}", auth_builder.launch().await);
}

// async fn handle_launch(arg: &LaucnhArgs) {
//     LaunchBuilder::builder()
//         .args(launch_args)
//         .launch(arg.jre)
//         .await;
// }
