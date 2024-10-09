use crate::application::services::node::NodeService;
use log::error;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn start(db_conn: Arc<Mutex<Connection>>) {
    // This runs only once, but could be a job running with CRON settings each N hours.

    tokio::spawn(async move {
        let service = NodeService::from(db_conn);
        if let Err(a) = service.refresh_node_rankings().await {
            error!("Failed to load nodes rankings from API.")
        }
    });
}
