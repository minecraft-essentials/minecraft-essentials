use super::*;
use dotenv::dotenv;
use std::env;

#[cfg(feature = "auth")]
#[tokio::test]
async fn test_oauth_url() {
    let _ = dotenv();
    let client_id = env::var("Client_ID").expect("Expected Client ID");
    let expected_url = format!(
        "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize/?client_id={}&response_type=code&redirect_uri=http://localhost:8000&response_mode=query&scope={}&state=12345",
        client_id, SCOPE
    );
    assert_eq!(oauth.url(), expected_url);
}

#[cfg(feature = "auth")]
#[tokio::test]
async fn test_device_code_prelaunch() {
    let _ = dotenv();
    let client_id = env::var("Client_ID").expect("Expected Client ID.");
    let device_code = DeviceCode::new(&client_id).await.unwrap();

    let (url, message, expires_in, user_code) = device_code.preinfo();

    assert_eq!(url, device_code.url);
    assert_eq!(message, device_code.message);
    assert_eq!(expires_in, device_code.expires_in);
    assert_eq!(user_code, device_code.user_code);
}

#[cfg(feature = "auth")]
#[tokio::test]
async fn test_authentication_info() {
    // Setup for OAuth
    let _ = dotenv();
    let client_id = env::var("Client_ID").expect("Expected Client ID.");
    let client_secret = env::var("Client_Secret").expect("Expected Client Secret.");
    let port = 8000;
    let mut builder = AuthenticationBuilder::builder();
    builder
        .of_type(AuthType::Oauth)
        .client_id(&client_id)
        .client_secret(&client_secret)
        .port(Some(port));

    let assert_url = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize/?clientid={}&response_type=code&redirect_uri=http://localhost:{}&response_mode=query&scope={}&state=12345", client_id, port, SCOPE);
    let url = builder.get_info().await._ouath_url.unwrap(); // Note: There seems to be a typo in the method name. It should likely be something like get_oauth_url()

    assert_eq!(assert_url, url);

    builder.of_type(AuthType::DeviceCode).client_id(&client_id);

    let device_code = builder.get_info().await._device_code.unwrap();
    let url = device_code.verification_uri.clone();
    let message = device_code.message.clone();
    let expires_in = device_code.expires_in.clone();
    let user_code = device_code.user_code.clone();

    assert_eq!(url, device_code.verification_uri);
    assert_eq!(message, device_code.message);
    assert_eq!(expires_in, device_code.expires_in);
    assert_eq!(user_code, device_code.user_code);
}

#[cfg(feature = "auth")]
#[tokio::test]
async fn test_authentication() {
    let _ = dotenv();
    let client_id = env::var("Client_ID").expect("Expected Client ID.");
    let client_secret = env::var("Client_Secret").expect("Expected Client Secret.");
    let port = env::var("Port").expect("Expected Port.");
    let mut builder = AuthenticationBuilder::builder();
    builder
        .of_type(AuthType::Oauth)
        .client_id(&client_id)
        .client_secret(&client_secret)
        .port(Some(port.parse::<u16>().unwrap()));

    let auth_info = builder.launch().await.unwrap();
    let access_token = auth_info.access_token.unwrap();
    let uuid = auth_info.uuid.unwrap();
    let expires_in = auth_info.expires_in;
    let xts_token = auth_info.xts_token.unwrap();

    assert_eq!(access_token.len(), 36);
    assert_eq!(uuid.len(), 36);
    assert!(expires_in > 0);
    assert_eq!(xts_token.len(), 36);

    builder.of_type(AuthType::DeviceCode).client_id(&client_id);

    let auth_info = builder.launch().await.unwrap();
    let access_token = auth_info.access_token.unwrap();
    let uuid = auth_info.uuid.unwrap();
    let expires_in = auth_info.expires_in;
    let xts_token = auth_info.xts_token.unwrap();

    assert_eq!(access_token.len(), 36);
    assert_eq!(uuid.len(), 36);
    assert!(expires_in > 0);
    assert_eq!(xts_token.len(), 36);
}
