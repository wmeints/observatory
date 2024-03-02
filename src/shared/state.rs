use std::sync::Arc;

#[derive(Debug)]
pub struct AppState {
    pub database_url: String,
}

impl AppState {
    /// Creates a new instance of the application state
    pub fn new(database_url: String) -> Arc<Self> {
        Arc::new(Self { database_url })
    }
}
