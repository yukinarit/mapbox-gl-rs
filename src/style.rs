use crate::layer::Layer;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum StyleOrRef {
    Style(Style),
    Ref(String),
}

impl Default for StyleOrRef {
    fn default() -> Self {
        Self::Ref("mapbox://styles/mapbox/streets-v11".into())
    }
}

impl From<StyleOrRef> for JsValue {
    fn from(val: StyleOrRef) -> Self {
        val.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
            .unwrap()
    }
}

impl wasm_bindgen::describe::WasmDescribe for StyleOrRef {
    fn describe() {
        JsValue::describe()
    }
}

impl IntoWasmAbi for StyleOrRef {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        self.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
            .unwrap()
            .into_abi()
    }
}

impl FromWasmAbi for StyleOrRef {
    type Abi = <JsValue as FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        serde_wasm_bindgen::from_value(JsValue::from_abi(js)).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Style {
    pub version: u32,
    pub sources: Sources,
    pub layers: Vec<Layer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glyphs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    // TODO: flesh out optional properties of Style spec
    //       see https://docs.mapbox.com/style-spec/reference/root
}

impl From<Style> for JsValue {
    fn from(val: Style) -> Self {
        val.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
            .unwrap()
    }
}

impl wasm_bindgen::describe::WasmDescribe for Style {
    fn describe() {
        JsValue::describe()
    }
}

impl IntoWasmAbi for Style {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        self.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
            .unwrap()
            .into_abi()
    }
}

impl FromWasmAbi for Style {
    type Abi = <JsValue as FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        serde_wasm_bindgen::from_value(JsValue::from_abi(js)).unwrap()
    }
}

pub type Sources = HashMap<String, Source>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Source {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl From<Source> for JsValue {
    fn from(val: Source) -> Self {
        serde_wasm_bindgen::to_value(&val).unwrap()
    }
}

impl wasm_bindgen::describe::WasmDescribe for Source {
    fn describe() {
        JsValue::describe()
    }
}

impl IntoWasmAbi for Source {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        serde_wasm_bindgen::to_value(&self).unwrap().into_abi()
    }
}

impl FromWasmAbi for Source {
    type Abi = <JsValue as FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        serde_wasm_bindgen::from_value(JsValue::from_abi(js)).unwrap()
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapboxSdkSupport {
    pub js: Option<String>,
    pub android: Option<String>,
    pub ios: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
