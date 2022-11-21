use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarkerOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_tolerance: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(skip)]
    pub element: Option<web_sys::HtmlElement>,
    //pub offset?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch_alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<u64>,
}

impl MarkerOptions {
    pub fn build(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

impl MarkerOptions {
    pub fn new() -> MarkerOptions {
        MarkerOptions::default()
    }
}

pub struct Marker {
    inner: crate::js::Marker,
    latlng: crate::LngLat,
}

impl Marker {
    pub fn new(latlng: crate::LngLat, options: MarkerOptions) -> Marker {
        let inner = crate::js::Marker::maker_new(options.build());
        Marker { inner, latlng }
    }

    pub fn add_to(&self, map: &crate::Map) {
        self.inner.setLngLat(&self.latlng.inner);
        self.inner.addTo(&map.inner)
    }

    pub fn remove(&self) {
        self.inner.remove()
    }
}
