use super::*;
use dotenv::dotenv;
use std::env;

#[cfg(feature = "custom-auth")]
#[tokio::test]
async fn test_oauth_url() {
    let _ = dotenv();
    let client_id = env::var("Client_ID").expect("Expected Client ID");
    let oauth = Oauth::new(&client_id, None);
    let params = format!("client_id={}&response_type=code&redirect_uri=http://localhost:8000&response_mode=query&scope={}&state=12345", client_id, SCOPE);
    let expected_url = format!(
        "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize/?{}",
        params
    );
    assert_eq!(oauth.url(), expected_url);
}

#[cfg(feature = "custom-auth")]
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