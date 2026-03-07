use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tauri::State;
use thunes_cli::context::Context;

#[tauri::command]
#[tracing::instrument(skip(database), ret(level = tracing::Level::DEBUG))]
pub async fn get_context(
    database: State<'_, tokio::sync::Mutex<Surreal<Db>>>,
) -> Result<Context, String> {
    let database = database.lock().await;

    thunes_cli::context::read(&database).await.map_err(|error| {
        error.trace();
        "failed to get context".to_string()
    })
}

#[tauri::command]
#[tracing::instrument(skip(database), ret(level = tracing::Level::DEBUG))]
pub async fn update_context(
    database: State<'_, tokio::sync::Mutex<Surreal<Db>>>,
    context: Context,
) -> Result<(), String> {
    let database = database.lock().await;

    thunes_cli::context::update(&database, context)
        .await
        .map_err(|error| {
            error.trace();
            "failed to update context".to_string()
        })
}
