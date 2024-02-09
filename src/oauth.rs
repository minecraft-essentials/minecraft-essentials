
#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

//! The server for OAauth login Server.

use actix_web::{web, App, HttpResponse, HttpServer};
use reqwest::Client;
use serde::Deserialize;
use tokio::sync::mpsc;

use crate::{errors::{OAuthError, TokenError}, SCOPE};

/// Infomation from the temporary http server.
#[derive(Deserialize, Debug)]
pub struct Info {
    /// The code
    pub code: Option<String>,
    /// The state
    pub state: Option<String>,
    // Error
    error: Option<String>,
    // Error description
    error_description: Option<String>,
}




#[derive(Deserialize, Debug)]
pub struct Token {
    pub token_type: String,
    pub scope: String,
    pub expires_in: u16,
    pub ext_expires_in: u16,
    pub access_token: String,
    pub refresh_token: String,
}


pub async fn server(port: u16) -> Result<Info, OAuthError> {
    let (tx, mut rx) = mpsc::channel::<Info>(1);

    let server = tokio::spawn(
        HttpServer::new(move || {
            App::new().app_data(actix_web::web::Data::new(tx.clone())).route(
                "/",
                web::get().to(|web::Query(info): web::Query<Info>, tx: web::Data<mpsc::Sender<Info>>| async move {
                    tx.try_send(info).unwrap();
                    HttpResponse::Ok().body("If you see this the authentification has ran into an error.")
                }),
            )
        })
        .bind(format!("127.0.0.1:{}", port)).map_err(|e| OAuthError::BindError(e.to_string()))?
        .workers(1)
        .run(),
    );

    let info = rx.recv().await.expect("server did not recive params");

    if info.error.as_ref().map_or(false, |s| !s.is_empty())
    && info
        .error_description
        .as_ref()
        .map_or(false, |s| !s.is_empty())
{
    let err = OAuthError::AuthenticationFailure(info.error_description.unwrap());
    return Err(err);
}

    server.abort();

    Ok(info)
}

pub async fn token(
    code: &str,
    client_id: &str,
    port: u16,
    client_secret: &str,
) -> Result<Token, TokenError> {
    let url = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/token");
    let client = Client::new();
    let body = format!("client_id={}&scope={}&redirect_uri=http://localhost:{}&grant_type=authorization_code&code={}&client_secret={}", client_id, SCOPE, port, code, client_secret);


    let result = client.post(url).body(body).send().await;


    let std::result::Result::Ok(response) = result else { println!("Part 1"); return Err(TokenError::ResponseError("Failed to send request".to_string()))}; 
        
    let text = response.text().await.map_err(|_| TokenError::ResponseError("Failed to send request".to_string()))?;
    let std::result::Result::Ok(token) = serde_json::from_str::<Token>(&text) else { return Err(TokenError::ResponseError("Failed to send request, Check your Client Secret.".to_string()))};
    std::result::Result::Ok(token)
}
