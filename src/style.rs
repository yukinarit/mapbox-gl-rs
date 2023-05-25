use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StyleOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ideograph_font_family: Option<String>,
}

impl StyleOptions {
    pub fn new() -> StyleOptions {
        StyleOptions::default()
    }
}
