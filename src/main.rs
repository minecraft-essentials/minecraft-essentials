use clap::{Args, Parser, Subcommand};
use minecraft_essentials::{DeviceCode, Oauth};

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
    DeviceCode(DeviceCodeArgs)
}

#[derive(Args)]
struct OauthArgs {
    client_id: String, 
    client_secret: String, 
    port: Option<u16>, 
    bedrockrelm: Option<bool>
}

#[derive(Args)]
struct DeviceCodeArgs {
    client_id: String, 
    bedrockrelm: bool
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Oauth(oauth_args) => handle_oauth(oauth_args).await,
        Commands::DeviceCode(device_code_args) => handle_device_code(device_code_args).await,
        Commands::Version {} => println!("{}", env!("CARGO_PKG_VERSION")),
    }
}

async fn handle_oauth(oauth_args: &OauthArgs) {
    let auth = Oauth::new(&oauth_args.client_id, Some(oauth_args.port.unwrap_or(8000)));
    println!("URL: {} \nWaiting for Login........", auth.url());
    match auth.launch(oauth_args.bedrockrelm.unwrap_or(false), &oauth_args.client_secret).await {
        Ok(authinfo) => {
            println!(
                "Bearer: {:?}, \n UUID: {:?}, \n Expire_in: {:?}, \n XtsToken: {:?}",
                authinfo.access_token, authinfo.uuid, authinfo.uuid, authinfo.xts_token
            );
        },
        Err(e) => eprintln!("Failed to launch: {}", e),
    }
}

async fn handle_device_code(device_code_args: &DeviceCodeArgs) {
   let auth = DeviceCode::new(&device_code_args.client_id).await;
   match auth {
    Ok(device_code) => {
        let (url, message, expires_in, user_code) = device_code.preinfo();
        println!("Info: URL: {}, Message: {}, Expires in: {}, User Code: {} \nWaiting for Login........", url, message, expires_in, user_code);

        let authinfo = device_code.launch(device_code_args.bedrockrelm);
    },
    Err(e) => {
        eprintln!("Failed to create DeviceCode: {}", e);
    }
}
}