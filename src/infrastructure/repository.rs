use crate::application::adapters::Satochi;
use crate::application::repository::NodesRepository;
use crate::application::types::Node;
use log::info;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct SqliteNodesRepository {
    pub connection: Arc<Mutex<Connection>>,
}

impl From<Arc<Mutex<Connection>>> for SqliteNodesRepository {
    fn from(value: Arc<Mutex<Connection>>) -> Self {
        Self { connection: value }
    }
}

impl NodesRepository for SqliteNodesRepository {
    async fn insert(&self, nodes: Vec<Node>) -> anyhow::Result<()> {
        if nodes.is_empty() {
            return Ok(());
        }

        // This is a trick to batch insert elements, because rustqlite doesn't provide any sugar for
        // batch operations we build a huge insertion string.
        // NOTE: This probably is not efficient and should be batched with another method.
        let inserted_values = nodes
            .into_iter()
            .map(|node| {
                format!(
                    "(\"{0}\", \"{1}\", \"{2}\", \"{3}\")",
                    node.public_key, node.alias, node.capacity, node.first_seen
                )
            })
            .collect::<Vec<String>>()
            .join(", ");

        let stmt = format!(
            "INSERT INTO nodes (public_key, alias, capacity, first_seen) VALUES {}",
            inserted_values
        );

        self.connection
            .clone()
            .lock()
            .await
            .execute(&stmt, [])
            .expect("TODO: panic message");

        info!("inserted new nodes successfully");
        Ok(())
    }

    async fn get_all(&self) -> anyhow::Result<Vec<Node>> {
        let query = "SELECT public_key, alias , capacity, first_seen FROM nodes";
        let db_conn = self.connection.clone();
        let conn = db_conn.lock().await;
        let mut statement = conn.prepare(query)?;
        let rows = statement.query_map([], |row| {
            Ok(Node {
                public_key: row.get(0).unwrap(),
                alias: row.get(1).unwrap(),
                capacity: row.get(2).map(|r: i64| Satochi::from(r)).unwrap(),
                first_seen: row.get(3).unwrap(),
            })
        })?;

        let rows = rows.flatten().collect::<Vec<Node>>();
        Ok(rows)
    }
}
