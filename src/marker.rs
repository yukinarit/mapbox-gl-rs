use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::event;

pub struct MarkerFactory {
    pub marker: Rc<Marker>,
    handle: Option<MarkerHandle>,
}

impl MarkerFactory {
    pub fn new(marker: Rc<Marker>) -> MarkerFactory {
        MarkerFactory {
            marker,
            handle: None,
        }
    }

    pub fn set_listener<F: MarkerEventListener + 'static>(&mut self, f: F) {
        let marker = Rc::downgrade(&self.marker);
        self.handle = Some(MarkerHandle::new(marker, f));
        let handle = self.handle.as_ref().unwrap();

        let inner = &self.marker.inner;
        inner.Marker_on("dragstart".into(), &handle.on_dragstart);
        inner.Marker_on("drag".into(), &handle.on_drag);
        inner.Marker_on("dragend".into(), &handle.on_dragend);
    }
}

macro_rules! impl_handler_for_marker{
    ($(($event:ident, $type:ident),)*) => {
impl MarkerHandle {
    pub fn new<F: MarkerEventListener + 'static>(marker: Weak<Marker>, f: F) -> MarkerHandle {
        let f = Rc::new(RefCell::new(f));
        MarkerHandle {
            $($event: impl_event_marker!(marker, f, $event, $type),)*
        }
    }
}
    }
}

macro_rules! impl_event_marker {
    ($m: ident, $f:ident, $event:ident, JsValue) => {
            Closure::new(enclose!(
                ($m, $f) move |value: JsValue| {
                    web_sys::console::debug_2(&JsValue::from(stringify!($event)), &value);
                    let Some(marker) = $m.upgrade() else {
                        warn!("Failed to get Map handle");
                        return;
                    };

                    match value.try_into() {
                        Ok(e) => {
                            if let Ok(mut f) = $f.try_borrow_mut() {
                                f.deref_mut().$event(marker, e);
                            } else {
                                error!("Event handler is being called somewhere.");
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize Event: {}", e);
                        }
                    }
                }
            ))
    };
}

impl_handler_for_marker! {
    (on_dragstart, JsValue),
    (on_drag, JsValue),
    (on_dragend, JsValue),
}

pub trait MarkerEventListener {
    fn on_dragstart(&mut self, _m: Rc<Marker>, _e: event::MapBaseEvent) {}
    fn on_drag(&mut self, _m: Rc<Marker>, _e: event::DragEvent) {}
    fn on_dragend(&mut self, _m: Rc<Marker>, _e: event::DragEvent) {}
}

pub struct MarkerHandle {
    on_dragstart: Closure<dyn Fn(JsValue)>,
    on_drag: Closure<dyn Fn(JsValue)>,
    on_dragend: Closure<dyn Fn(JsValue)>,
}

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
    pub fn build(mut self) -> JsValue {
        let obj: js_sys::Object = serde_wasm_bindgen::to_value(&self).unwrap().into();
        if let Some(element) = self.element.take() {
            js_sys::Reflect::set(&obj, &JsValue::from_str("element"), &element).unwrap();
            obj.into()
        } else {
            obj.into()
        }
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

    pub fn get_lnglat(&self) -> LngLat {
        LngLat {
            inner: self.inner.getLngLat(),
        }
    }
}
