use std::sync::Arc;

use axum::http;
use diesel::dsl::exists;
use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::schema::words::dsl::words;
use crate::schema::words::id as word_table_id;
use crate::schema::words::user_id as word_table_user_id;
use crate::AppState;

pub async fn execute_in_db<F, R>(app_state: Arc<AppState>, f: F) -> R
where
    F: FnOnce(&mut PgConnection) -> R + Send + 'static,
    R: Send + 'static,
{
    app_state
        .db
        .get()
        .await
        .expect("Failed to get database connection")
        .interact(f)
        .await
        .expect("Error interacting with the database")
}

pub async fn validate_user_access(
    app_state: Arc<AppState>,
    user_id: &String,
    word_id: &Uuid,
) -> Result<(), http::StatusCode> {
    let uuid = word_id.clone();
    let user_id = user_id.clone();
    let exists = execute_in_db(app_state, move |conn| {
        diesel::select(exists(
            words.filter(word_table_id.eq(uuid).and(word_table_user_id.eq(user_id))),
        ))
        .get_result::<bool>(conn)
        .expect("Error loading data")
    })
    .await;

    if exists {
        Ok(())
    } else {
        Err(http::StatusCode::NOT_FOUND)
    }
}
