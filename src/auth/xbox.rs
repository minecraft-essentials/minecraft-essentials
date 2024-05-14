#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

use crate::async_trait_alias::*;
use reqwest::{
    header::{self, HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE},
    Client,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::errors::{XTSError, XboxError};
#[derive(Deserialize, Debug)]
pub struct Xui {
    pub uhs: String,
}

#[derive(Deserialize, Debug)]
pub struct DisplayClaims {
    pub xui: Vec<Xui>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct XblOutput {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub display_claims: DisplayClaims,
}

pub fn xbl(token: &str) -> impl AsyncSendSync<Result<XblOutput, XboxError>> {
    let client = Client::new();
    let url = format!("https://user.auth.xboxlive.com/user/authenticate");
    let rps_ticket = format!("d={}", token);
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let body = json!({
       "Properties": {
           "AuthMethod": "RPS",
           "SiteName": "user.auth.xboxlive.com",
           "RpsTicket": rps_ticket,
       },
       "RelyingParty": "http://auth.xboxlive.com",
       "TokenType": "JWT"
    });

    xbl_internal(client, url, headers, body)
}

async fn xbl_internal(
    client: Client,
    url: String,
    headers: HeaderMap,
    body: Value,
) -> Result<XblOutput, XboxError> {
    let result = client
        .post(url)
        .headers(headers)
        .body(body.to_string())
        .send()
        .await;

    let std::result::Result::Ok(response) = result else {
        println!("Part 1");
        return Err(XboxError::ResponseError(
            "Failed to send request".to_string(),
        ));
    };
    let text = response
        .text()
        .await
        .map_err(|_| XboxError::ResponseError("Failed to send request".to_string()))?;

    let std::result::Result::Ok(token) = serde_json::from_str::<XblOutput>(&text) else {
        return Err(XboxError::ResponseError(
            "Failed to send request".to_string(),
        ));
    };
    std::result::Result::Ok(token)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct XtsOutput {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub display_claims: DisplayClaims,
}

pub fn xsts_token(
    xbl_token: &str,
    bedrock_rel: bool,
) -> impl AsyncSendSync<Result<XtsOutput, XTSError>> {
    let url = format!("https://xsts.auth.xboxlive.com/xsts/authorize");
    let bedrock_party = "https://pocket.realms.minecraft.net/";
    let java_party = "rp://api.minecraftservices.com/";
    let party = if bedrock_rel == true {
        bedrock_party
    } else {
        java_party
    };

    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "Accept",
        header::HeaderValue::from_static("application/json"),
    );
    let body = json!({
       "Properties": {
           "SandboxId": "RETAIL",
           "UserTokens": [
            format!("{}",xbl_token)
           ]
       },
       "RelyingParty": party,
       "TokenType": "JWT"
    });

    xsts_internal(client, url, body, headers)
}

async fn xsts_internal(
    client: Client,
    url: String,
    body: Value,
    headers: HeaderMap,
) -> Result<XtsOutput, XTSError> {
    let result = client
        .post(url)
        .body(body.to_string())
        .headers(headers)
        .send()
        .await;

    let std::result::Result::Ok(response) = result else {
        println!("Part  1");
        return Err(XTSError::ResponseError(
            "Failed to send request".to_string(),
        ));
    };
    let text = response
        .text()
        .await
        .map_err(|_| XTSError::ResponseError("Failed to read response text".to_string()))?;
    let std::result::Result::Ok(token) = serde_json::from_str::<XtsOutput>(&text) else {
        println!("Part  2");
        return Err(XTSError::ResponseError(
            "Failed to parse response".to_string(),
        ));
    };
    Ok(token)
}
