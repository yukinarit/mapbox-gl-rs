use serde::{Deserialize, Serialize};

use crate::layer::Layer;

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

#[derive(Deserialize, Debug)]
pub struct MapboxSdkSupport {
    pub js: Option<String>,
    pub android: Option<String>,
    pub ios: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    #[serde(rename = "mapbox:origin")]
    pub mapbox_origin: Option<String>,
    #[serde(rename = "mapbox:autocomposite")]
    pub mapbox_autocomposite: Option<bool>,
    #[serde(rename = "mapbox:type")]
    pub mapbox_type: Option<String>,
    #[serde(rename = "mapbox:sdk-support")]
    pub mapbox_sdk_support: Option<MapboxSdkSupport>,
}

#[derive(Deserialize, Debug)]
pub struct Source {
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub source_type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Style {
    pub version: Option<u32>,
    pub name: Option<String>,
    pub metadata: Option<Metadata>,
    pub center: Option<Vec<f64>>,
    pub zoom: Option<f64>,
    pub bearing: Option<f64>,
    pub pitch: Option<f64>,
    pub sprite: Option<String>,
    pub glyphs: Option<String>,
    pub layers: Option<Vec<Layer>>,
    pub created: Option<String>,
    pub id: Option<String>,
    pub modified: Option<String>,
    pub owner: Option<String>,
    pub visibility: Option<String>,
    pub draft: Option<bool>,
}
