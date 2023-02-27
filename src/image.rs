use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub struct Image {
    pub inner: JsValue,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<(f64, f64, f64, f64)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pixcel_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sdf: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stretch_x: Vec<(f64, f64)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stretch_y: Vec<(f64, f64)>,
}
