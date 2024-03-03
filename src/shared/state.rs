use std::sync::Arc;
use mongodb::Client;
use crate::shared::database::connect_database;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database_client: Arc<Client>
}

impl AppState {
    /// Creates a new instance of the application state
    pub async fn from_configuration(database_url: String) -> Result<Self> {
        let client = connect_database(database_url.as_str()).await?;

        let state = AppState {
            database_client: Arc::new(client)
        };

        Ok(state)
    }
}
