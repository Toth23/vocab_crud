use std::sync::Arc;

use diesel::SqliteConnection;

use crate::AppState;

pub async fn execute_in_db<F, R>(app_state: Arc<AppState>, f: F) -> R
    where
        F: FnOnce(&mut SqliteConnection) -> R + Send + 'static,
        R: Send + 'static,
{
    app_state.db.get().await.expect("Failed to get database connection")
        .interact(f)
        .await.expect("Error interacting with the database")
}
