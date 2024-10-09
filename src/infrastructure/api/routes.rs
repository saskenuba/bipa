use crate::application::adapters::Bitcoin;
use crate::application::repository::NodesRepository;
use crate::infrastructure::api::output_dto::NodeFormatted;
use crate::infrastructure::repository::SqliteNodesRepository;
use crate::infrastructure::server::AppState;
use axum::extract::State;
use axum::Json;

pub async fn nodes(State(state): State<AppState>) -> Json<Vec<NodeFormatted<Bitcoin>>> {
    let repo = SqliteNodesRepository::from(state.db_conn);
    let nodes = repo
        .get_all()
        .await
        .unwrap()
        .into_iter()
        .map(NodeFormatted::from)
        .collect();
    Json(nodes)
}
