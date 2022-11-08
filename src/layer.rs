use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Layer {
    pub id: String,
    pub r#type: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint: Option<Paint>,
}

impl Layer {
    pub fn new(
        id: impl Into<String>,
        r#type: impl Into<String>,
        source: impl Into<String>,
    ) -> Layer {
        Layer {
            id: id.into(),
            r#type: r#type.into(),
            source: source.into(),
            layout: None,
            paint: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Layout {
    pub line_join: String,
    pub line_cap: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Paint {
    pub line_color: String,
    pub line_width: u32,
}
