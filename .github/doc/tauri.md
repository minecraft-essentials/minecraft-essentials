# Tauri Support

# OAuth

To use our custom authentication OAuth you have to spawn a new thread so we have provided an example below to look at if your having troubles.

```rs
#[tauri::command]
use tokio::sync::mpsc;

async fn auth() -> Result<CustomAuthData, String> {
    let (tx, mut rx) = mpsc::channel(1);
    tauri::async_runtime::spawn(async move {
        let result = handle_auth().await.map_err(|e| e.to_string());
        let _ = tx.send(result).await; // Send the result back through the channel
    });

    // Receive the result from the channel
    match rx.recv().await {
        Some(result) => result, // Return the result from handle_auth
        None => Err("No result received from handle_auth".to_string()), // Handle the case where no result is received
    }
}


async fn handle_auth() -> Result<CustomAuthData, Box<dyn std::error::Error>> {
    let auth = Oauth::new("ClientID", None);
    let window_url = auth.url();

    let _ = open::that(window_url);

    let auth_info = auth.launch(false, "ClientSecret").await?;

    Ok(auth_info)
}



```