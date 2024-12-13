use rust_forum::{
    database::{establish_connection, set_database_up},
    server::start_server,
    AppState,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app_state = AppState {
        database_connection: establish_connection().await,
    };
    set_database_up(&app_state.database_connection).await;

    start_server(app_state).await;
}
