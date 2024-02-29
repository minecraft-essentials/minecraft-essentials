use clap::{Command, Parser, Subcommand};
use minecraft_essentials::{CustomAuthData, DeviceCode, Oauth};

#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"), about = "A Cli for a minecraft client essentials library", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Version {},
    Oauth {client_id: String, client_secret: String, port: Option<u16>, bedrockrelm: Option<bool>},
    DeviceCode {client_id: String, bedrockrelm: bool}
}


#[tokio::main] // Assuming you're using Tokio as your async runtime
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Oauth { client_id, client_secret, port, bedrockrelm } => {
    
            let auth = Oauth::new(&client_id, Some(port));

            println!("Your Authentification URL is Here: {}", auth.url());

            println!("Waiting for Login........");
            let authinfo: CustomAuthData = auth
                .launch(Option<*bedrockrelm>, &client_secret)
                .await
                .expect("Failed to launch");

            println!(
                "Bearer: {:?}, \n UUID: {:?}, \n Expire_in: {:?}, \n XtsToken: {:?}",
                authinfo.access_token, authinfo.uuid, authinfo.uuid, authinfo.xts_token
            );
        },
        Commands::DeviceCode { client_id, bedrockrelm } => {

        },
        Commands::Version {} => {
            println!("{}", env!("CARGO_PKG_VERSION"))
        },
        _ => unreachable!(),
    }
}
