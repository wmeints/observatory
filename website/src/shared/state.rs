use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
}

impl AppState {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}
