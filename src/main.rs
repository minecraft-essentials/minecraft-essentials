use clap::{arg, Command};
use minecraft_essentials::{CustomAuthData, DeviceCode, Oauth};

fn cli() -> Command {
    Command::new("minecraft-essentials")
        .about("A Cli for a minecraft client essentials library")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("oauth")
                .about("OAuth authentication method")
                .arg(arg!(<Client_ID> "Client Id").required(true))
                .arg(arg!(<Client_Secret> "Client Secret").required(true))
                .arg(arg!(<Server_Port> "Port").required(false))
                .arg(arg!(<Bedrock_rel> "Bedrock Relm").required(false)),
        )
        .subcommand(
            Command::new("devicecode")
                .about("Device Code authentication method")
                .arg(arg!(<Client_ID> "Client Id").required(true))
                .arg(arg!(<Bedrock_rel> "Bedrock Relm").required(false)),
        )
        .subcommand(Command::new("version").about("Tells the version of the package"))
}

#[tokio::main] // Assuming you're using Tokio as your async runtime
async fn main() {
    let commands = cli().get_matches();
    match commands.subcommand() {
        Some(("oauth", sub_match)) => {
            let client_id = sub_match
                .get_one::<String>("Client_ID")
                .expect("Expected Client Id");
            let client_secret = sub_match
                .get_one::<String>("Client_Secret")
                .expect("Expected Client Secret");
            let port = sub_match.get_one::<u16>("Server_Port").unwrap_or(&8000);
            let bedrock = sub_match.get_one::<bool>("").unwrap_or(&false);

            let auth = Oauth::new(client_id, Some(*port));

            println!("Your Authentification URL is Here: {}", auth.url());

            println!("Waiting for Login........");
            let authinfo: CustomAuthData = auth
                .launch(*bedrock, &client_secret)
                .await
                .expect("Failed to launch");

            println!(
                "Bearer: {:?}, \n UUID: {:?}, \n Expire_in: {:?}, \n XtsToken: {:?}",
                authinfo.access_token, authinfo.uuid, authinfo.uuid, authinfo.xts_token
            );
        }
        Some(("devicecode", sub_match)) => {
            let client_id = sub_match
                .get_one::<String>("Client_ID")
                .expect("Expected Client Id");
            let bedrock = sub_match.get_one::<bool>("").unwrap_or(&false);

            let auth_future = DeviceCode::new(client_id);
            let auth = auth_future
                .await
                .expect("Failed to create DeviceCode instance");

            println!("Instuctions Info: {:?}", auth.preinfo());

            println!("Waiting for Login........");
            let authinfo: CustomAuthData = auth.launch(*bedrock).await.expect("Failed to launch");

            println!(
                "Bearer: {:?}, \n UUID: {:?}, \n Expire_in: {:?}, \n XtsToken: {:?}",
                authinfo.access_token, authinfo.uuid, authinfo.uuid, authinfo.xts_token
            );
        }
        Some(("version", _sub_match)) => {
            println!("{}", env!("CARGO_PKG_VERSION"))
        }
        _ => unreachable!(),
    }
}
