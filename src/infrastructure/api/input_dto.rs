use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeRankingBaseDTO {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub alias: String,
    pub channels: i64,
    pub capacity: i64,
    #[serde(rename = "firstSeen")]
    pub first_seen: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    pub city: Option<NodeCityDTO>,
    pub country: Option<NodeCountryDTO>,
    pub iso_code: Option<String>,
    pub subdivision: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeCityDTO {
    pub de: Option<String>,
    pub en: Option<String>,
    pub es: Option<String>,
    pub fr: Option<String>,
    pub ja: Option<String>,
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<String>,
    pub ru: Option<String>,
    #[serde(rename = "zh-CN")]
    pub zh_cn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeCountryDTO {
    pub de: String,
    pub en: String,
    pub es: String,
    pub fr: String,
    pub ja: String,
    #[serde(rename = "pt-BR")]
    pub pt_br: String,
    pub ru: String,
    #[serde(rename = "zh-CN")]
    pub zh_cn: String,
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::api::input_dto::NodeRankingBaseDTO;
    #[test]
    fn successfully_can_deserialize() {
        serde_json::from_str::<Vec<NodeRankingBaseDTO>>(include_str!(
            "../../../resources/sample.json"
        ))
        .unwrap();
    }
}
