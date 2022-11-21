use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PopupOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
}

impl PopupOptions {
    pub fn build(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

impl PopupOptions {
    pub fn new() -> PopupOptions {
        PopupOptions::default()
    }
}

pub struct Popup {
    inner: crate::js::Popup,
    latlng: crate::LngLat,
}

impl Popup {
    pub fn new(latlng: crate::LngLat, options: PopupOptions) -> Popup {
        let inner = crate::js::Popup::Popup_new(options.build());
        Popup { inner, latlng }
    }

    pub fn add_to(&self, map: &crate::Map) {
        self.inner.Popup_setLngLat(&self.latlng.inner);
        self.inner.Popup_addTo(&map.inner)
    }

    pub fn set_html(&self, html: impl Into<String>) {
        self.inner.Popup_setHTML(html.into());
    }
}
