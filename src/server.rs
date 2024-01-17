#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

//! The server for OAauth login Technically

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::str;
use tokio::sync::mpsc;

/// Infomation from the temporary http server.
#[derive(Deserialize)]
pub struct Info {
    /// The code
    pub code: String,
    /// The state
    pub state: String,
}

pub async fn launch(port: u16) -> std::io::Result<Info> {
    let (tx, mut rx) = mpsc::channel::<Info>(1);

    let server = tokio::spawn(
        HttpServer::new(move || {
            App::new().app_data(tx.clone()).route(
                "/",
                web::get().to(|web::Query(info): web::Query<Info>, tx: web::Data<mpsc::Sender<Info>>| async move {
                    tx.try_send(info).unwrap();
                    HttpResponse::Ok()
                }),
            )
        })
        .bind(format!("127.0.0.1:{}", port))?
        .workers(1)
        .run(),
    );

    let info = rx.recv().await.expect("server did not launch");

    server.abort();

    Ok(info)
}
