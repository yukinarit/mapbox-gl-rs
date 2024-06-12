use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
use wasm_bindgen::prelude::*;

use crate::*;

macro_rules! impl_handler_for_marker {
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
                let Some(marker) = $m.upgrade() else {
                    warn!("Failed to get a marker handle");
                    return;
                };

                match value.try_into() {
                    Ok(e) => {
                        if let Ok(mut f) = $f.try_borrow_mut() {
                            f.deref_mut().$event(marker, e);
                        } else {
                            error!("Marker event handler is being called somewhere.");
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

#[allow(unused_variables)]
pub trait MarkerEventListener {
    fn on_dragstart(&mut self, map: Rc<Marker>, e: event::MapBaseEvent) {}
    fn on_drag(&mut self, map: Rc<Marker>, e: event::DragEvent) {}
    fn on_dragend(&mut self, map: Rc<Marker>, e: event::DragEvent) {}
}

struct NoopListener;

impl MarkerEventListener for NoopListener {}

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
    inner: js::Marker,
    lnglat: LngLat,
    handle: RefCell<Option<MarkerHandle>>,
}

impl Marker {
    pub fn new(lnglat: LngLat) -> Rc<Marker> {
        Self::with_listener(lnglat, MarkerOptions::default(), NoopListener {})
    }

    pub fn with_options(lnglat: LngLat, options: MarkerOptions) -> Rc<Marker> {
        Self::with_listener(lnglat, options, NoopListener {})
    }

    pub fn with_listener<F>(lnglat: LngLat, options: MarkerOptions, f: F) -> Rc<Marker>
    where
        F: MarkerEventListener + 'static,
    {
        let marker = Rc::new(Marker {
            inner: js::Marker::maker_new(options.build()),
            lnglat,
            handle: RefCell::new(None),
        });

        let handle = MarkerHandle::new(Rc::downgrade(&marker), f);
        let inner = &marker.inner;
        inner.Marker_on("dragstart".into(), &handle.on_dragstart);
        inner.Marker_on("drag".into(), &handle.on_drag);
        inner.Marker_on("dragend".into(), &handle.on_dragend);
        marker.handle.borrow_mut().replace(handle);

        marker
    }

    pub(crate) fn add_to(&self, map: &Map) {
        self.inner.setLngLat(&self.lnglat.inner);
        self.inner.addTo(&map.inner)
    }

    pub(crate) fn remove(&self) {
        self.inner.remove()
    }

    pub fn get_lnglat(&self) -> LngLat {
        LngLat {
            inner: self.inner.getLngLat(),
        }
    }
    pub fn set_lnglat(&self, pos: &LngLat) {
        self.inner.setLngLat(&pos.inner)
    }
}
