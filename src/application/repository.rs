use crate::application::types::Node;
use std::future::Future;

pub trait NodesRepository {
    fn insert(&self, nodes: Vec<Node>) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn get_all(&self) -> impl Future<Output = anyhow::Result<Vec<Node>>> + Send;
}
