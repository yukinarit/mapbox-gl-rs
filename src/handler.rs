use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub struct BoxZoomHandler {
    pub inner: crate::js::BoxZoomHandler,
}

impl BoxZoomHandler {
    pub fn new(map: &crate::Map) -> Self {
        BoxZoomHandler {
            inner: crate::js::BoxZoomHandler::BoxZoomHandler_new(
                &map.inner,
                serde_wasm_bindgen::to_value(&BoxZoomHandlerOption::default()).unwrap(),
            ),
        }
    }

    pub fn enable(&self) {
        self.inner.BoxZoomHandler_enable()
    }

    pub fn disable(&self) {
        self.inner.BoxZoomHandler_disable()
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.BoxZoomHandler_isEnabled()
    }

    pub fn is_active(&self) -> bool {
        self.inner.BoxZoomHandler_isActive()
    }

    //pub fn disable_roration(&self) {
    //    self.inner.BoxZoomHandler_disableRotation()
    //}

    //pub fn enable_roration(&self) {
    //    self.inner.BoxZoomHandler_enableRotation()
    //}
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxZoomHandlerOption {
    click_tolerance: u32,
}

impl Default for BoxZoomHandlerOption {
    fn default() -> Self {
        BoxZoomHandlerOption { click_tolerance: 1 }
    }
}
