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

//! The server for OAauth login Server.

use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::anyhow;
use reqwest::{header::{HeaderMap, HeaderValue, CONTENT_TYPE, HOST}, Client};
use serde::Deserialize;
use std::str;
use tokio::sync::mpsc;

use crate::SCOPE;

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
pub struct TokenInfo {
    pub access_token: String
}

#[derive(Deserialize, Debug)]
struct Token {
    access_token: String,
    expires_in: u16,
    refresh_token: String,
    id_token: String,
}



pub async fn server(port: u16) -> Result<Info, anyhow::Error> {
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
        .bind(format!("127.0.0.1:{}", port))?
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
        let err = format!("\x1b[33mFailed to authenticate.\x1b[0m").to_string();
        return Err(anyhow!(
            "\x1b[31mResponse: {}\x1b[0m",
            info.error_description.unwrap()
        )
        .context(err)
        .into());
    }

    server.abort();

    Ok(info)
}



pub async fn token(
    code: &str,
    client_id: &str,
    port: u16,
    client_secret: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/token?client_id={}&scope={}&code={}&redirect_uri=https://localhost:{}&grant_type=authorization_code&client_secret={}", client_id, SCOPE, code, port, client_secret);
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(HOST, HeaderValue::from_static("https://login.microsoftonline.com"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
   
    let response = client
        .post(url)
        .headers(headers)
        .send()
        .await?;

    let text = response.text().await?;
        println!("{:?}", text);

    Ok(())
}