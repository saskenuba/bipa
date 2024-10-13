use crate::infrastructure::api::routes;
use crate::infrastructure::jobs;
use axum::routing::get;
use axum::Router;
use log::info;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!();
}

#[derive(Clone, Debug)]
pub struct AppState {
    // An enhancement would be to create a pool of connections to sqlite and each time a handler
    // uses one, it would take from the pool and put it back after use.
    pub db_conn: Arc<Mutex<Connection>>,
}

pub async fn serve() {
    let mut connection =
        Connection::open_in_memory().expect("SQLite is bundled and should not fail to start.");
    {
        embedded::migrations::runner().run(&mut connection).unwrap();
    }
    info!("sqlite migrations ran successfully.");

    let db_connection = Arc::new(Mutex::new(connection));
    jobs::start(db_connection.clone());
    info!("jobs started ...");

    let router = Router::new()
        .route("/nodes", get(routes::nodes))
        .with_state(AppState {
            db_conn: db_connection.clone(),
        });

    info!("serving routes ...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap()
}
