pub mod code;
pub mod mojang;
pub mod oauth;
pub mod xbox;

pub async fn oauth(
    port: u16,
    client_id: &str,
    client_secret: &str,
    bedrock_relm: bool,
) -> Result<mojang::AuthInfo, Box<dyn std::error::Error>> {
    let http_server = oauth::server(port)?.await?;
    let token = oauth::token(
        http_server
            .code
            .expect("\x1b[31mXbox Expected code.\x1b[0m")
            .as_str(),
        client_id,
        port,
        client_secret,
    )
    .await?;
    let xbox = xbox::xbl(&token.access_token).await?;
    let xts = xbox::xsts_token(&xbox.token, bedrock_relm).await?;

    if bedrock_relm {
        Ok(crate::CustomAuthData {
            access_token: "null".to_string(),
            uuid: "null".to_string(),
            expires_in: 0,
            xts_token: Some(xts.token),
        })
    } else {
        Ok(mojang::token(&xbox.display_claims.xui[0].uhs, &xts.token).await?)
    }
}

pub async fn DeviceCode(
    client_id: &str,
    bedrock_relm: bool,
) -> Result<mojang::AuthInfo, Box<dyn std::error::Error>> {
    let response_data = code::device_authentication_code(client_id).await?;
    let token = code::authenticate_device(response_data.device_code.as_str(), client_id).await?;
    let xbox = xbox::xbl(&token.token).await?;
    let xts = xbox::xsts_token(&xbox.token, bedrock_relm).await?;
    if bedrock_relm {
        Ok(crate::CustomAuthData {
            access_token: "null".to_string(),
            uuid: "null".to_string(),
            expires_in: 0,
            xts_token: Some(xts.token),
        })
    } else {
        Ok(mojang::token(&xbox.display_claims.xui[0].uhs, &xts.token).await?)
    }
}
