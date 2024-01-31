use dotenv_vault::dotenv;
use minecraft_essentials::*;
use std::env;

#[allow(dead_code)]
async fn oauth() {
    let _ = dotenv();
    let client_id = env::var("Client_ID").expect("Expected Client ID");
    let client_secret = env::var("Client_Secret").expect("Expected Client Secret");
    let auth = Oauth::new(&client_id, None);

    println!("{}", auth.url());

    let auth_info = auth.launch(false, &client_secret).await;

    println!("{:?}", auth_info)
}

#[allow(dead_code)]
async fn device() {
    let _ = dotenv();
    let client_id = env::var("Client_ID").expect("Expected Client ID");
    let auth = DeviceCode::new(&client_id).await.expect("Failed to create device code");

    println!("{:?}", auth.prelaunch());

    let auth_info = auth.launch(false).await;

    println!("{:?}", auth_info)
}

#[allow(dead_code)]
async fn launch_maven() {}
#[allow(dead_code)]
async fn launch_gradle() {}


#[tokio::main]
async fn main() {
    // oauth().await;
    device().await;
}
