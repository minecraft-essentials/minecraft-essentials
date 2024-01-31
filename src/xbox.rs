/*
* Copyright (C) 2024 Mincraft-essnetials

* This program is free software: you can redistribute it and/or modify it
* under the terms of the GNU Affero General Public License as published by
* the Free Software Foundation, either version 3 of the License, or (at your
* option) any later version.

* This program is distributed in the hope that it will be useful, but WITHOUT
* ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
* FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public
* License for more details.

* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

use reqwest::{header::{self, HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE}, Client};
use serde::Deserialize;
use serde_json::json;

use crate::errors::{XboxError, XTSError};
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

pub async fn xbl(token: &str) -> Result<XblOutput, XboxError> {
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
    let result = client.post(url).headers(headers).body(body.to_string()).send().await;

    let std::result::Result::Ok(response) = result else { println!("Part 1"); return Err(XboxError {})};
    let text = response.text().await.map_err(|_| XboxError {})?;

    let std::result::Result::Ok(token) = serde_json::from_str::<XblOutput>(&text) else { println!("Part 2"); return Err(XboxError {})};
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

pub async fn xsts_token(
    xbl_token: &str,
    bedrock_rel: bool,
) -> Result<XtsOutput, XTSError> {
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
    let result = client.post(url).body(body.to_string()).headers(headers).send().await;
    let std::result::Result::Ok(response) = result else { println!("Part 1"); return Err(XTSError {})};
    let text = response.text().await.map_err(|_| XTSError {})?;
    let std::result::Result::Ok(token) = serde_json::from_str::<XtsOutput>(&text) else { println!("Part 2"); return Err(XTSError {})};
    Ok(token)
}
