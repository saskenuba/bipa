use crate::application::types::Node;

pub trait NodesRepository {
    fn insert(&self, nodes: Vec<Node>) -> anyhow::Result<()>;
    async fn get_all(&self) -> anyhow::Result<Vec<Node>>;
}
