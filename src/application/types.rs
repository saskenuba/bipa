use crate::application::adapters::Satochi;
use crate::infrastructure::api::input_dto::NodeRankingBaseDTO;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;
use time::format_description::well_known::iso8601::{Config, EncodedConfig, TimePrecision};
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub public_key: String,
    pub alias: String,
    /// Capacity are represented in Satochi units.
    pub capacity: Satochi,
    pub first_seen: String,
}

const ISO8601_CONFIG: EncodedConfig = {
    let cfg_default = Config::DEFAULT;
    let cfg = cfg_default.set_time_precision(TimePrecision::Second {
        decimal_digits: NonZeroU8::new(2),
    });
    cfg.encode()
};

impl From<NodeRankingBaseDTO> for Node {
    fn from(value: NodeRankingBaseDTO) -> Self {
        let formatter: Iso8601<ISO8601_CONFIG> = Iso8601;

        let first_seen = OffsetDateTime::from_unix_timestamp(value.first_seen)
            .unwrap()
            .format(&formatter)
            .unwrap();

        Self {
            public_key: value.public_key,
            alias: value.alias,
            capacity: value.capacity.into(),
            first_seen,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::types::Node;
    use crate::infrastructure::api::input_dto::NodeRankingBaseDTO;

    #[test]
    fn successfully_parse_and_transform() {
        let result = serde_json::from_str::<Vec<NodeRankingBaseDTO>>(include_str!(
            "../../resources/sample.json"
        ))
        .unwrap();

        let node = Node::from(result[0].clone());
        println!("{:?}", node);
    }
}
