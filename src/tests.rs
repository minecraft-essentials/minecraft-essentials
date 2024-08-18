use super::*;
use dotenv::dotenv;
use std::env;

#[cfg(feature = "auth")]
#[tokio::test]
async fn test_oauth_url() {
    let _ = dotenv();
    let client_id = env::var("Client_ID").expect("Expected Client ID");
    let port = 8000;
    let expected_url = format!(
        "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize/?client_id={}&response_type=code&redirect_uri=http://localhost:{}&response_mode=query&scope={}&state=12345",
        client_id, port, SCOPE
    );
    let oauth = Oauth::new(&client_id, Some(port));
    let url = oauth.url().await;

    assert_eq!(url, expected_url);
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
        .port(port);

    let assert_url = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize/?client_id={}&response_type=code&redirect_uri=http://localhost:{}&response_mode=query&scope={}&state=12345", client_id, port, SCOPE);
    let url = builder.get_info().await.ouath_url.unwrap(); // Note: There seems to be a typo in the method name. It should likely be something like get_oauth_url()

    assert_eq!(assert_url, url);

    builder.of_type(AuthType::DeviceCode).client_id(&client_id);

    let device_code = builder.get_info().await.device_code.unwrap();
    let url = device_code.verification_uri.clone();
    let message = device_code.message.clone();
    let expires_in = device_code.expires_in.clone();
    let user_code = device_code.user_code.clone();

    assert_eq!(url, device_code.verification_uri);
    assert_eq!(message, device_code.message);
    assert_eq!(expires_in, device_code.expires_in);
    assert_eq!(user_code, device_code.user_code);
}

struct ProjcetTest {
    name: String,
    description: String,
    license: String,
}

#[cfg(feature = "modrinth")]
#[tokio::test]
async fn test_modrinth_project() {
    let _ = dotenv();

    let project = Modrinth::init(
        GithubModrinth {
            owner: "modrinth".to_string(),
            repo: "modrinth".to_string(),
            project_version: "0.2.12".to_string(),
            access_token: None,
        },
        "contact@minecraft-essentials.com".to_string(),
    )
    .get_project("fabric-api")
    .await;

    let project_expected = ProjcetTest {
        name: "Fabric API".to_string(),
        description: "Lightweight and modular API providing common hooks and intercompatibility measures utilized by mods using the Fabric toolchain.".to_string(),
        license: "Apache License 2.0".to_string(),
    };

    // Recive a response modrnth project
    if let Ok(project) = project {
        assert_eq!(project.title, project_expected.name);
        assert_eq!(project.description, project_expected.description);
        assert_eq!(project.license.name, project_expected.license);
    } else {
        panic!("Failed to get project: {:?}", project);
    }
}
