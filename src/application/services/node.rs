use crate::application::repository::NodesRepository;
use crate::application::types::Node;
use crate::infrastructure::api::input_dto::NodeRankingBaseDTO;
use crate::infrastructure::repository::SqliteNodesRepository;
use futures_util::TryFutureExt;
use log::info;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

const NODES_ENDPOINT: &str = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

pub struct NodeService {
    pub repository: SqliteNodesRepository,
}

impl From<Arc<Mutex<Connection>>> for NodeService {
    fn from(value: Arc<Mutex<Connection>>) -> Self {
        Self {
            repository: SqliteNodesRepository { connection: value },
        }
    }
}

impl NodeService {
    pub async fn refresh_node_rankings(&self) -> anyhow::Result<()> {
        info!("fetching nodes connectivity rankings ...");
        let response = reqwest::get(NODES_ENDPOINT)
            .and_then(|response| response.json::<Vec<NodeRankingBaseDTO>>())
            .map_ok(|nodes| nodes.into_iter().map(Node::from).collect::<Vec<_>>())
            .await?;

        self.repository.insert(response)
    }
}
