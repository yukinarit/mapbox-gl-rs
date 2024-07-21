use crate::Layer;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub version: u32,
    pub sources: Sources,
    pub layers: Vec<Layer>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub r#type: String,
    pub tiles: Vec<String>,
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
