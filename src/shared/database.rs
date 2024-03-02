use anyhow::Result;
use mongodb::{options::ClientOptions, Client};

/// Connects to the mongo database
pub async fn connect_database(url: &str) -> Result<Client> {
    let client_options = ClientOptions::parse(url).await?;
    let client = Client::with_options(client_options)?;

    Ok(client)
}
