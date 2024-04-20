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

#[cfg(feature = "custom-launch")]
#[tokio::test]
async fn test_custom_launch() {
    let _ = dotenv();
    let args = vec![
        "--uuid:uuidtest".to_string(),
        "--token:tokentest".to_string(),
    ];
    let args_join = args.join(" ");
    let jre = Some(PathBuf::from("/test/java"));
    let java_exe = "/test/java";

    let launch =
        Launch::new(args, java_exe.to_string(), jre.clone(), Some(false)).expect("Expected Launch");
    let (launch_args, launch_java_exe, launch_jre) = launch.info();
    assert_eq!(args_join, launch_args);
    assert_eq!(java_exe, launch_java_exe);
    assert_eq!(jre, launch_jre.clone());
}
