use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use minecraft_essentials::{
    structs::{GameArguments, JavaArguments, QuickPlayArguments},
    ArgsDescriptive, AuthType, AuthenticationBuilder, LaunchBuilder,
};

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
    min_memory: Option<i32>,
    max_memory: i32,
    launcher_name: Option<String>,
    launcher_version: Option<String>,
    class_path: Option<String>,

    client_id: Option<String>,
    username: Option<String>,
    version: Option<String>,
    uuid: Option<String>,
    game_directory: Option<PathBuf>,
    width: Option<i32>,
    height: Option<i32>,

    quick_play_singleplayer: Option<String>,
    quick_play_multiplayer: Option<String>,
}

#[derive(Args, Clone)]
struct WindowSize {
    width: i32,
    height: i32,
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
        Commands::Launch(arg) => handle_launch(arg).await,
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

async fn handle_launch(arg: &LaucnhArgs) {
    let mut builder = LaunchBuilder::builder();

    let quick_play_arguments = if let Some(singleplayer) = arg.quick_play_singleplayer.clone() {
        QuickPlayArguments::SinglePlayer(singleplayer)
    } else if let Some(multiplayer) = arg.quick_play_multiplayer.clone() {
        QuickPlayArguments::MultiPlayer(multiplayer)
    } else {
        QuickPlayArguments::None
    };

    let args = minecraft_essentials::Args::Descriptive(ArgsDescriptive {
        game_args: Some(GameArguments {
            window_size: Some((arg.width.unwrap_or(1920), arg.height.unwrap_or(1080))),
            // Handling client_id correctly to avoid borrow checker errors
            client_id: Some(
                arg.client_id
                    .as_deref()
                    .map(|s| s.to_owned())
                    .unwrap_or_else(|| String::from("")),
            ),
            username: <Option<std::string::String> as Clone>::clone(&arg.username)
                .map(|s| s.to_owned()),
            version: Some(
                <Option<std::string::String> as Clone>::clone(&arg.version)
                    .map(|s| s.to_owned())
                    .unwrap_or_else(|| "1.20.1".to_string()),
            ),
            game_directory: arg.game_directory.as_ref().map(|pb| {
                <PathBuf as Clone>::clone(&pb)
                    .into_os_string()
                    .into_string()
                    .unwrap_or(String::new())
            }),
            uuid: arg.uuid.as_ref().map(|s| s.to_owned()),
            quick_play: Some(quick_play_arguments),
        }),
        java_args: JavaArguments {
            min_memory: arg.min_memory.map(|mem| mem as i32),
            max_memory: arg.max_memory,
            launcher_name: arg.launcher_name.as_deref().map(|s| s.to_owned()),
            launcher_version: arg.launcher_version.as_deref().map(|s| s.to_owned()),
            class_path: arg.class_path.as_deref().map(|s| s.to_owned()),
        },
    });

    builder.java_args(args);
    builder.launch().await;
}
