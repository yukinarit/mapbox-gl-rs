#![doc = include_str!("../README.md")]
pub mod event;
pub mod handler;
mod js;
pub mod layer;
pub mod marker;
pub mod popup;
pub mod source;

use anyhow::Result;
use enclose::enclose;
use log::*;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    ops::DerefMut,
    rc::Rc,
    sync::{Arc, Weak},
};
use wasm_bindgen::{prelude::*, JsCast};

pub use handler::BoxZoomHandler;
pub use layer::Layer;
pub use marker::{Marker, MarkerOptions};
pub use popup::{Popup, PopupOptions};
pub use source::GeoJsonSource;

const DEFAULT_STYLE: &'static str = "mapbox://styles/mapbox/streets-v11";

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomAttribution {
    Single(String),
    Multiple(Vec<String>),
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapOptions {
    access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    antialias: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribution_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bearing: Option<i64>,
    // TODO bounds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    box_zoom: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    center: Option<LatLng>,
    #[serde(skip_serializing_if = "Option::is_none")]
    click_tolerance: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collect_resource_timing: Option<bool>,

    container: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    cooperative_gestures: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_source_collisions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_attribution: Option<CustomAttribution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    double_click_zoom: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drag_pan: Option<bool>,
    style: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    projection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_expired_tiles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    render_world_copies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_zoom: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zoom: Option<f64>,
}

#[wasm_bindgen]
impl MapOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(access_token: String, container: String) -> MapOptions {
        MapOptions {
            access_token: access_token,
            antialias: None,
            attribution_control: None,
            bearing: None,
            box_zoom: None,
            center: None,
            click_tolerance: None,
            collect_resource_timing: None,
            container: container,
            cooperative_gestures: None,
            cross_source_collisions: None,
            custom_attribution: None,
            double_click_zoom: None,
            drag_pan: None,
            style: DEFAULT_STYLE.into(),
            projection: None,
            refresh_expired_tiles: None,
            render_world_copies: None,
            scroll_zoom: None,
            test_mode: None,
            zoom: None,
        }
    }

    pub fn style(mut self, style: String) -> MapOptions {
        self.style = style;
        self
    }

    pub fn projection(mut self, projection: String) -> MapOptions {
        self.projection = Some(projection);
        self
    }

    pub fn center(mut self, latlng: LatLng) -> MapOptions {
        self.center = Some(latlng);
        self
    }

    pub fn zoom(mut self, zoom: f64) -> MapOptions {
        self.zoom = Some(zoom);
        self
    }

    pub fn build(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

pub trait MapEventListner {
    fn on_resize(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_remove(&mut self, _m: &Map, _e: event::MapBaseEvent) {}

    // Interaction
    fn on_mousedown(&mut self, _m: &Map, _e: event::MapMouseEvent) {}
    fn on_mouseup(&mut self, _m: &Map, _e: event::MapMouseEvent) {}
    fn on_preclick(&mut self, _m: &Map, _e: event::MapMouseEvent) {}
    fn on_click(&mut self, _m: &Map, _e: event::MapMouseEvent) {}
    fn on_dblclick(&mut self, _m: &Map, _e: event::MapMouseEvent) {}
    fn on_mousemove(&mut self, _m: &Map, _e: event::MapMouseEvent) {}
    fn on_mouseover(&mut self, _m: &Map, _e: event::MapMouseEvent) {}
    fn on_mouseenter(&mut self, _m: &Map, _e: event::MapMouseEvent) {}
    fn on_mouseleave(&mut self, _m: &Map, _e: event::MapMouseEvent) {}
    fn on_mouseout(&mut self, _m: &Map, _e: event::MapMouseEvent) {}
    fn on_contextmenu(&mut self, _m: &Map, _e: event::MapMouseEvent) {}
    fn on_touchstart(&mut self, _m: &Map, _e: event::MapTouchEvent) {}
    fn on_touchend(&mut self, _m: &Map, _e: event::MapTouchEvent) {}
    fn on_touchcancel(&mut self, _m: &Map, _e: event::MapTouchEvent) {}
    fn on_wheel(&mut self, _m: &Map, _e: event::MapWheelEvent) {}

    // Movement
    fn on_movestart(&mut self, _m: &Map, _e: event::DragEvent) {}
    fn on_move(&mut self, _m: &Map, _e: event::MapEvent) {}
    fn on_moveend(&mut self, _m: &Map, _e: event::DragEvent) {}
    fn on_dragstart(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_drag(&mut self, _m: &Map, _e: event::DragEvent) {}
    fn on_dragend(&mut self, _m: &Map, _e: event::DragEvent) {}
    fn on_zoomstart(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_zoom(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_zoomend(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_rotatestart(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_rotate(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_rotateend(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_pitchstart(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_pitch(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_pitchend(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_boxzoomstart(&mut self, _m: &Map, _e: event::MapBoxZoomEvent) {}
    fn on_boxzoomend(&mut self, _m: &Map, _e: event::MapBoxZoomEvent) {}
    fn on_boxzoomcancel(&mut self, _m: &Map, _e: event::MapBoxZoomEvent) {}

    // Lifecycle
    fn on_load(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_render(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_idle(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_error(&mut self, _m: &Map, _message: String) {}
    fn on_webglcontextlost(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_webglcontextrestored(&mut self, _m: &Map, _e: event::MapBaseEvent) {}

    // Data loading
    fn on_data(&mut self, _m: &Map, _e: event::MapDataEvent) {}
    fn on_styledata(&mut self, _m: &Map, _e: event::MapDataEvent) {}
    fn on_sourcedata(&mut self, _m: &Map, _e: event::MapDataEvent) {}
    fn on_dataloading(&mut self, _m: &Map, _e: event::MapDataEvent) {}
    fn on_styledataloading(&mut self, _m: &Map, _e: event::MapDataEvent) {}
    fn on_sourcedataloading(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
    fn on_styleimagemissing(&mut self, _m: &Map, _e: event::MapBaseEvent) {}
}

pub struct Handle {
    on_resize: Closure<dyn Fn(JsValue)>,
    on_remove: Closure<dyn Fn(JsValue)>,

    // Interaction
    on_mousedown: Closure<dyn Fn(JsValue)>,
    on_mouseup: Closure<dyn Fn(JsValue)>,
    on_preclick: Closure<dyn Fn(JsValue)>,
    on_click: Closure<dyn Fn(JsValue)>,
    on_dblclick: Closure<dyn Fn(JsValue)>,
    on_mousemove: Closure<dyn Fn(JsValue)>,
    on_mouseover: Closure<dyn Fn(JsValue)>,
    on_mouseenter: Closure<dyn Fn(JsValue)>,
    on_mouseleave: Closure<dyn Fn(JsValue)>,
    on_mouseout: Closure<dyn Fn(JsValue)>,
    on_contextmenu: Closure<dyn Fn(JsValue)>,
    on_touchstart: Closure<dyn Fn(JsValue)>,
    on_touchend: Closure<dyn Fn(JsValue)>,
    on_touchcancel: Closure<dyn Fn(JsValue)>,
    on_wheel: Closure<dyn Fn(JsValue)>,

    // Movement
    on_movestart: Closure<dyn Fn(JsValue)>,
    on_move: Closure<dyn Fn(JsValue)>,
    on_moveend: Closure<dyn Fn(JsValue)>,
    on_dragstart: Closure<dyn Fn(JsValue)>,
    on_drag: Closure<dyn Fn(JsValue)>,
    on_dragend: Closure<dyn Fn(JsValue)>,
    on_zoomstart: Closure<dyn Fn(JsValue)>,
    on_zoom: Closure<dyn Fn(JsValue)>,
    on_zoomend: Closure<dyn Fn(JsValue)>,
    on_rotatestart: Closure<dyn Fn(JsValue)>,
    on_rotate: Closure<dyn Fn(JsValue)>,
    on_rotateend: Closure<dyn Fn(JsValue)>,
    on_pitchstart: Closure<dyn Fn(JsValue)>,
    on_pitch: Closure<dyn Fn(JsValue)>,
    on_pitchend: Closure<dyn Fn(JsValue)>,
    on_boxzoomstart: Closure<dyn Fn(JsValue)>,
    on_boxzoomend: Closure<dyn Fn(JsValue)>,
    on_boxzoomcancel: Closure<dyn Fn(JsValue)>,

    // Lifecycle
    on_load: Closure<dyn Fn(JsValue)>,
    on_render: Closure<dyn Fn(JsValue)>,
    on_idle: Closure<dyn Fn(JsValue)>,
    on_error: Closure<dyn Fn(JsValue)>,
    on_webglcontextlost: Closure<dyn Fn(JsValue)>,
    on_webglcontextrestored: Closure<dyn Fn(JsValue)>,

    // Data loading
    on_data: Closure<dyn Fn(JsValue)>,
    on_styledata: Closure<dyn Fn(JsValue)>,
    on_sourcedata: Closure<dyn Fn(JsValue)>,
    on_dataloading: Closure<dyn Fn(JsValue)>,
    on_styledataloading: Closure<dyn Fn(JsValue)>,
    on_sourcedataloading: Closure<dyn Fn(JsValue)>,
    on_styleimagemissing: Closure<dyn Fn(JsValue)>,
}

macro_rules! impl_handler{
    ($(($event:ident, $type:ident),)*) => {
impl Handle {
    pub fn new<F: MapEventListner + 'static>(map: Weak<Map>, f: F) -> Handle {
        let f = Rc::new(RefCell::new(f));
        Handle {
            $($event: impl_event!(map, f, $event, $type),)*
        }
    }
}
    }
}

macro_rules! impl_event {
    ($m: ident, $f:ident, $event:ident, JsValue) => {
            Closure::new(enclose!(
                ($m, $f) move |value: JsValue| {
                    web_sys::console::log_2(&JsValue::from(stringify!($event)), &value);
                    match  value.try_into() {
                        Ok(e) => {
                            if let Ok(mut f) = $f.try_borrow_mut() {
                                f.deref_mut().$event($m.upgrade().unwrap().as_ref(), e);
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
    ($m:ident, $f:ident, $event:ident, String) => {
            Closure::new(enclose!(
                ($m, $f) move |value: JsValue| {
                    web_sys::console::log_2(&JsValue::from(stringify!($event)), &value);
                    //$f.$event($m.upgrade().unwrap().as_ref(), value.as_string().unwrap());
                    if let Ok(mut f) = $f.try_borrow_mut() {
                        f.deref_mut().$event($m.upgrade().unwrap().as_ref(), value.as_string().unwrap());
                    } else {
                        error!("Event handler is being called somewhere.");
                    }
                }
            ))
    };
}

impl_handler! {
    (on_resize, JsValue),
    (on_remove, JsValue),

    // Interaction
    (on_mousedown,  JsValue),
    (on_mouseup,    JsValue),
    (on_preclick,   JsValue),
    (on_click,      JsValue),
    (on_dblclick,   JsValue),
    (on_mousemove,  JsValue),
    (on_mouseover,  JsValue),
    (on_mouseenter, JsValue),
    (on_mouseleave, JsValue),
    (on_mouseout,   JsValue),
    (on_contextmenu,JsValue),
    (on_touchstart, JsValue),
    (on_touchend,   JsValue),
    (on_touchcancel,JsValue),
    (on_wheel,   JsValue),

    // Movement
    (on_movestart, JsValue),
    (on_move, JsValue),
    (on_moveend, JsValue),
    (on_dragstart, JsValue),
    (on_drag, JsValue),
    (on_dragend, JsValue),
    (on_zoomstart,JsValue),
    (on_zoom,JsValue),
    (on_zoomend, JsValue),
    (on_rotatestart, JsValue),
    (on_rotate, JsValue),
    (on_rotateend, JsValue),
    (on_pitchstart, JsValue),
    (on_pitch, JsValue),
    (on_pitchend, JsValue),
    (on_boxzoomstart, JsValue),
    (on_boxzoomend, JsValue),
    (on_boxzoomcancel,JsValue),

    // Lifecycle
    (on_load, JsValue),
    (on_render, JsValue),
    (on_idle, JsValue),
    (on_error, String),
    (on_webglcontextlost, JsValue),
    (on_webglcontextrestored, JsValue),

    // Data loading
    (on_data, JsValue),
    (on_styledata, JsValue),
    (on_sourcedata, JsValue),
    (on_dataloading, JsValue),
    (on_styledataloading, JsValue),
    (on_sourcedataloading, JsValue),
    (on_styleimagemissing, JsValue),
}

pub struct MapFactory {
    pub map: Arc<Map>,
    handle: Option<Handle>,
}

pub struct Map {
    pub(crate) inner: crate::js::Map,
}

impl MapFactory {
    pub fn new(options: MapOptions) -> Result<MapFactory> {
        let inner = crate::js::Map::new(options.build());

        Ok(MapFactory {
            map: Arc::new(Map { inner }),
            handle: None,
        })
    }

    pub fn set_listener<F: MapEventListner + 'static>(&mut self, f: F) {
        let map = Arc::downgrade(&self.map);
        self.handle = Some(Handle::new(map, f));
        let handle = self.handle.as_ref().unwrap();

        let inner = &self.map.inner;
        inner.on("resize".into(), &handle.on_resize);
        inner.on("remove".into(), &handle.on_remove);

        // Interaction
        inner.on("mousedown".into(), &handle.on_mousedown);
        inner.on("mouseup".into(), &handle.on_mouseup);
        inner.on("preclick".into(), &handle.on_preclick);
        inner.on("click".into(), &handle.on_click);
        inner.on("dblclick".into(), &handle.on_dblclick);
        inner.on("mousemove".into(), &handle.on_mousemove);
        inner.on("mouseover".into(), &handle.on_mouseover);
        inner.on("mouseenter".into(), &handle.on_mouseenter);
        inner.on("mouseleave".into(), &handle.on_mouseleave);
        inner.on("mouseout".into(), &handle.on_mouseout);
        inner.on("contextmenu".into(), &handle.on_contextmenu);
        inner.on("touchstart".into(), &handle.on_touchstart);
        inner.on("touchend".into(), &handle.on_touchend);
        inner.on("touchcancel".into(), &handle.on_touchcancel);
        inner.on("wheel".into(), &handle.on_wheel);

        // Movement
        inner.on("movestart".into(), &handle.on_movestart);
        inner.on("move".into(), &handle.on_move);
        inner.on("moveend".into(), &handle.on_moveend);
        inner.on("dragstart".into(), &handle.on_dragstart);
        inner.on("drag".into(), &handle.on_drag);
        inner.on("dragend".into(), &handle.on_dragend);
        inner.on("zoomstart".into(), &handle.on_zoomstart);
        inner.on("zoom".into(), &handle.on_zoom);
        inner.on("zoomend".into(), &handle.on_zoomend);
        inner.on("rotatestart".into(), &handle.on_rotatestart);
        inner.on("rotate".into(), &handle.on_rotate);
        inner.on("rotateend".into(), &handle.on_rotateend);
        inner.on("pitchstart".into(), &handle.on_pitchstart);
        inner.on("pitch".into(), &handle.on_pitch);
        inner.on("pitchend".into(), &handle.on_pitchend);
        inner.on("boxzoomstart".into(), &handle.on_boxzoomstart);
        inner.on("boxzoomend".into(), &handle.on_boxzoomend);
        inner.on("boxzoomcancel".into(), &handle.on_boxzoomcancel);

        // Lifecycle
        inner.on("load".into(), &handle.on_load);
        inner.on("render".into(), &handle.on_render);
        inner.on("idle".into(), &handle.on_idle);
        inner.on("error".into(), &handle.on_error);
        inner.on("webglcontextlost".into(), &handle.on_webglcontextlost);
        inner.on(
            "webglcontextrestored".into(),
            &handle.on_webglcontextrestored,
        );

        // Data loading
        inner.on("data".into(), &handle.on_data);
        inner.on("styledata".into(), &handle.on_styledata);
        inner.on("sourcedata".into(), &handle.on_sourcedata);
        inner.on("dataloading".into(), &handle.on_dataloading);
        inner.on("styledataloading".into(), &handle.on_styledataloading);
        inner.on("sourcedataloading".into(), &handle.on_sourcedataloading);
        inner.on("styleimagemissing".into(), &handle.on_styleimagemissing);
    }
}

#[derive(Debug, Clone)]
enum HandlerType {
    BoxZoom,
}

impl ToString for HandlerType {
    fn to_string(&self) -> String {
        use HandlerType::*;
        match self {
            BoxZoom => "boxZoom".into(),
        }
    }
}

impl Map {
    pub fn get_container(&self) -> web_sys::HtmlElement {
        self.inner.getContainer()
    }

    pub fn get_min_zoom(&self) -> f64 {
        self.inner.getMinZoom()
    }

    pub fn is_moving(&self) -> bool {
        self.inner.isMoving()
    }

    pub fn is_zooming(&self) -> bool {
        self.inner.isZooming()
    }

    pub fn is_rotating(&self) -> bool {
        self.inner.isRotating()
    }

    pub fn show_tile_boundaries(&self, value: bool) {
        self.inner.set_showTileBoundaries(value);
    }

    pub fn show_terrain_wireframe(&self, value: bool) {
        self.inner.set_showTerrainWireframe(value);
    }

    pub fn show_padding(&self, value: bool) {
        self.inner.set_showPadding(value);
    }

    pub fn show_collision_boxes(&self, value: bool) {
        self.inner.set_showCollisionBoxes(value);
    }

    pub fn get_box_zoom_handler(&self) -> Option<handler::BoxZoomHandler> {
        let value = self.inner.get_handler(&HandlerType::BoxZoom.to_string());
        web_sys::console::log_2(&JsValue::from("Get BoxZoomHandler: "), &value);

        let inner: js::BoxZoomHandler = value.unchecked_into();
        web_sys::console::log_1(&inner);
        Some(handler::BoxZoomHandler { inner })
    }

    pub fn set_box_zoom_handler(&self, handler: handler::BoxZoomHandler) {
        let handler::BoxZoomHandler { inner } = handler;
        self.inner
            .set_handler(&HandlerType::BoxZoom.to_string(), inner);
    }

    pub fn add_layer(&self, layer: &layer::Layer) -> Result<()> {
        self.inner.Map_addLayer(
            serde_wasm_bindgen::to_value(&layer)
                .map_err(|_| anyhow::anyhow!("Failed to convert Layer"))?,
        );

        Ok(())
    }

    pub fn add_geojson_source(
        &self,
        id: impl Into<String>,
        data: geojson::GeoJson,
    ) -> anyhow::Result<()> {
        let ser = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        let data = (&source::GeoJsonSourceSpec::new(data))
            .serialize(&ser)
            .map_err(|_| anyhow::anyhow!("Failed to convert GeoJson"))?;

        web_sys::console::log_1(&data);

        self.inner.Map_addSource(id.into(), data);

        Ok(())
    }

    pub fn get_geojson_source(&self, id: impl Into<String>) -> Option<source::GeoJsonSource> {
        let source = self.inner.Map_getSource(id.into());
        web_sys::console::log_2(&JsValue::from("Source: "), &source);

        if !source.is_undefined() {
            Some(GeoJsonSource {
                inner: source.unchecked_into(),
            })
        } else {
            None
        }
    }

    pub fn pan_to(&self, latlng: LatLng) {
        self.inner
            .Map_panTo(latlng, JsValue::undefined(), JsValue::undefined());
    }

    pub fn loaded(&self) -> bool {
        self.inner.Map_loaded()
    }
}
