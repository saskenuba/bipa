use crate::application::adapters::Bitcoin;
use crate::application::types::Node;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeFormatted<F>
where
    F: Display,
{
    pub public_key: String,
    pub alias: String,
    pub capacity: F,
    pub first_seen: String,
}

impl From<Node> for NodeFormatted<Bitcoin> {
    fn from(value: Node) -> Self {
        Self {
            public_key: value.public_key,
            alias: value.alias,
            capacity: Bitcoin::from(value.capacity),
            first_seen: value.first_seen,
        }
    }
}
