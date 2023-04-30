#![doc = include_str!("../README.md")]
mod callback;
pub mod error;
pub mod event;
pub mod handler;
pub mod image;
mod js;
pub mod layer;
pub mod marker;
pub mod popup;
pub mod source;

use anyhow::Result;
use enclose::enclose;
use log::*;
use marker::MarkerFactory;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    ops::DerefMut,
    rc::{Rc, Weak},
};
use uuid::Uuid;
use wasm_bindgen::{prelude::*, JsCast};

use callback::CallbackStore;
pub use error::Error;
pub use handler::BoxZoomHandler;
pub use image::{Image, ImageOptions};
pub use layer::{Layer, Layout, LayoutProperty};
pub use marker::{Marker, MarkerOptions};
pub use popup::{Popup, PopupOptions};
pub use source::GeoJsonSource;

const DEFAULT_STYLE: &str = "mapbox://styles/mapbox/streets-v11";

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct LngLat {
    #[serde(
        serialize_with = "serialize_lnglat",
        deserialize_with = "deserialize_lnglat"
    )]
    inner: js::LngLat,
}

impl LngLat {
    pub fn lng(&self) -> f64 {
        self.inner.lng()
    }

    pub fn lat(&self) -> f64 {
        self.inner.lat()
    }

    pub fn set_lng(&self, v: f64) {
        self.inner.set_lng(v);
    }

    pub fn set_lat(&self, v: f64) {
        self.inner.set_lat(v);
    }
}

impl std::fmt::Debug for LngLat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LngLat")
            .field("lng", &self.inner.lng())
            .field("lat", &self.inner.lat())
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LngLatValue {
    lng: f64,
    lat: f64,
}

fn serialize_lnglat<S>(lnglat: &js::LngLat, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let value = LngLatValue {
        lng: lnglat.lng(),
        lat: lnglat.lat(),
    };

    value.serialize(ser)
}

fn deserialize_lnglat<'de, D>(de: D) -> Result<js::LngLat, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = LngLatValue::deserialize(de)?;
    Ok(js::LngLat::new(value.lng, value.lat))
}

impl LngLat {
    pub fn new(lng: f64, lat: f64) -> LngLat {
        LngLat {
            inner: js::LngLat::new(lng, lat),
        }
    }

    pub fn wrap(&self) -> LngLat {
        let wrapped = self.inner.wrap();
        LngLat { inner: wrapped }
    }

    pub fn to_array(&self) -> Vec<f64> {
        self.inner.toArray()
    }

    pub fn distance_to(&self, lnglat: LngLat) -> f64 {
        self.inner.distanceTo(&lnglat.inner)
    }

    pub fn to_bounds(&self, radius: f64) -> LngLatBounds {
        let bbox = self.inner.toBounds(radius);
        LngLatBounds { inner: bbox }
    }
}

impl From<js::LngLat> for LngLat {
    fn from(lnglat: js::LngLat) -> Self {
        LngLat { inner: lnglat }
    }
}

impl ToString for LngLat {
    fn to_string(&self) -> String {
        self.inner.toString()
    }
}

#[wasm_bindgen]
pub struct LngLatBounds {
    inner: js::LngLatBounds,
}

impl std::fmt::Debug for LngLatBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LngLatBounds")
            .field("sw", &self.get_south_west())
            .field("ne", &self.get_north_east())
            .finish()
    }
}

impl LngLatBounds {
    pub fn set_north_east(&self) -> LngLat {
        LngLat {
            inner: self.inner.getNorthEast(),
        }
    }

    pub fn get_south_west(&self) -> LngLat {
        LngLat {
            inner: self.inner.getSouthWest(),
        }
    }

    pub fn get_north_east(&self) -> LngLat {
        LngLat {
            inner: self.inner.getNorthEast(),
        }
    }

    pub fn get_center(&self) -> LngLat {
        LngLat {
            inner: self.inner.getCenter(),
        }
    }
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
    center: Option<LngLat>,
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
            access_token,
            antialias: None,
            attribution_control: None,
            bearing: None,
            box_zoom: None,
            center: None,
            click_tolerance: None,
            collect_resource_timing: None,
            container,
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

    pub fn center(mut self, latlng: LngLat) -> MapOptions {
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

pub trait MapEventListener {
    fn on_resize(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_remove(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}

    // Interaction
    fn on_mousedown(&mut self, _m: Rc<Map>, _e: event::MapMouseEvent) {}
    fn on_mouseup(&mut self, _m: Rc<Map>, _e: event::MapMouseEvent) {}
    fn on_preclick(&mut self, _m: Rc<Map>, _e: event::MapMouseEvent) {}
    fn on_click(&mut self, _m: Rc<Map>, _e: event::MapMouseEvent) {}
    fn on_dblclick(&mut self, _m: Rc<Map>, _e: event::MapMouseEvent) {}
    fn on_mousemove(&mut self, _m: Rc<Map>, _e: event::MapMouseEvent) {}
    fn on_mouseover(&mut self, _m: Rc<Map>, _e: event::MapMouseEvent) {}
    fn on_mouseenter(&mut self, _m: Rc<Map>, _e: event::MapMouseEvent) {}
    fn on_mouseleave(&mut self, _m: Rc<Map>, _e: event::MapMouseEvent) {}
    fn on_mouseout(&mut self, _m: Rc<Map>, _e: event::MapMouseEvent) {}
    fn on_contextmenu(&mut self, _m: Rc<Map>, _e: event::MapMouseEvent) {}
    fn on_touchstart(&mut self, _m: Rc<Map>, _e: event::MapTouchEvent) {}
    fn on_touchend(&mut self, _m: Rc<Map>, _e: event::MapTouchEvent) {}
    fn on_touchcancel(&mut self, _m: Rc<Map>, _e: event::MapTouchEvent) {}
    fn on_wheel(&mut self, _m: Rc<Map>, _e: event::MapWheelEvent) {}

    // Movement
    fn on_movestart(&mut self, _m: Rc<Map>, _e: event::DragEvent) {}
    fn on_move(&mut self, _m: Rc<Map>, _e: event::MapEvent) {}
    fn on_moveend(&mut self, _m: Rc<Map>, _e: event::DragEvent) {}
    fn on_dragstart(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_drag(&mut self, _m: Rc<Map>, _e: event::DragEvent) {}
    fn on_dragend(&mut self, _m: Rc<Map>, _e: event::DragEvent) {}
    fn on_zoomstart(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_zoom(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_zoomend(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_rotatestart(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_rotate(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_rotateend(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_pitchstart(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_pitch(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_pitchend(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_boxzoomstart(&mut self, _m: Rc<Map>, _e: event::MapBoxZoomEvent) {}
    fn on_boxzoomend(&mut self, _m: Rc<Map>, _e: event::MapBoxZoomEvent) {}
    fn on_boxzoomcancel(&mut self, _m: Rc<Map>, _e: event::MapBoxZoomEvent) {}

    // Lifecycle
    fn on_load(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_render(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_idle(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_error(&mut self, _m: Rc<Map>, _message: String) {}
    fn on_webglcontextlost(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_webglcontextrestored(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}

    // Data loading
    fn on_data(&mut self, _m: Rc<Map>, _e: event::MapDataEvent) {}
    fn on_styledata(&mut self, _m: Rc<Map>, _e: event::MapDataEvent) {}
    fn on_sourcedata(&mut self, _m: Rc<Map>, _e: event::MapDataEvent) {}
    fn on_dataloading(&mut self, _m: Rc<Map>, _e: event::MapDataEvent) {}
    fn on_styledataloading(&mut self, _m: Rc<Map>, _e: event::MapDataEvent) {}
    fn on_sourcedataloading(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
    fn on_styleimagemissing(&mut self, _m: Rc<Map>, _e: event::MapBaseEvent) {}
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
    pub fn new<F: MapEventListener + 'static>(map: Weak<Map>, f: F) -> Handle {
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
                    web_sys::console::debug_2(&JsValue::from(stringify!($event)), &value);

                    let Some(map) = $m.upgrade() else {
                        warn!("Failed to get Map handle");
                        return;
                    };

                    match  value.try_into() {
                        Ok(e) => {
                            if let Ok(mut f) = $f.try_borrow_mut() {
                                f.deref_mut().$event(map, e);
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
                    web_sys::console::warn_2(&JsValue::from(stringify!($event)), &value);

                    let Some(map) = $m.upgrade() else {
                        warn!("Failed to get Map handle");
                        return;
                    };

                    if let Ok(mut f) = $f.try_borrow_mut() {
                        f.deref_mut().$event(map, value.as_string().unwrap());
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
    pub map: Rc<Map>,
    handle: Option<Handle>,
    pub marker: Option<MarkerFactory>,
}

pub struct Map {
    pub(crate) inner: crate::js::Map,
    pub(crate) image_cbs: CallbackStore<dyn FnMut(JsValue, JsValue) + 'static>,
}

impl MapFactory {
    pub fn new(options: MapOptions) -> Result<MapFactory> {
        let options = options.build();

        let inner = crate::js::Map::new(options);

        Ok(MapFactory {
            map: Rc::new(Map {
                inner,
                image_cbs: CallbackStore::new(),
            }),
            handle: None,
            marker: None,
        })
    }

    pub fn set_listener<F: MapEventListener + 'static>(&mut self, f: F) {
        let map = Rc::downgrade(&self.map);
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

    pub fn set_marker(&mut self, marker: MarkerFactory) {
        self.marker = Some(marker);
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

    pub fn get_bounds(&self) -> anyhow::Result<LngLatBounds> {
        let bbox = self.inner.getBounds();
        Ok(LngLatBounds { inner: bbox })
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
        let inner: js::BoxZoomHandler = value.unchecked_into();
        Some(handler::BoxZoomHandler { inner })
    }

    pub fn set_box_zoom_handler(&self, handler: handler::BoxZoomHandler) {
        let handler::BoxZoomHandler { inner } = handler;
        self.inner
            .set_handler(&HandlerType::BoxZoom.to_string(), inner);
    }

    /// Add image resource.
    pub fn add_image(
        &self,
        id: impl Into<String>,
        image: crate::image::Image,
        options: crate::image::ImageOptions,
    ) -> crate::error::Result<()> {
        self.inner.addImage(
            id.into(),
            image.inner,
            serde_wasm_bindgen::to_value(&options).map_err(Error::from)?,
        );

        Ok(())
    }

    /// Update an existing image in a style.
    pub fn update_image(
        &self,
        id: impl Into<String>,
        image: crate::image::Image,
    ) -> crate::error::Result<()> {
        self.inner.updateImage(id.into(), image.inner);

        Ok(())
    }

    /// Check whether or not an image with a specific ID exists in the style.
    pub fn has_image(&self, id: impl Into<String>) -> bool {
        self.inner.hasImage(id.into())
    }

    /// Remove an image from a style.
    pub fn remove_image(&self, id: impl Into<String>) {
        self.inner.removeImage(id.into())
    }

    /// Load an image from an external URL.
    pub fn load_image(
        &self,
        url: impl Into<String>,
        mut callback: impl FnMut(crate::error::Result<crate::image::Image>) + 'static,
    ) {
        let callback_id = Uuid::new_v4();
        let cbs = self.image_cbs.clone();

        let callback = Closure::new(move |e: JsValue, image| {
            if e.is_null() {
                callback(Ok(crate::image::Image { inner: image }));
            } else {
                callback(Err(Error::LoadImage(e)));
            }

            if let Err(e) = cbs.remove(&callback_id) {
                warn!("{e}");
            }
        });

        self.inner.loadImage(url.into(), &callback);

        if let Err(e) = self.image_cbs.add(callback_id, callback) {
            warn!("{e}");
        }
    }

    /// Returns an Array of strings containing the IDs of all images currently available in the map.
    pub fn list_images(&self) -> error::Result<Vec<String>> {
        let images = self.inner.listImages();
        serde_wasm_bindgen::from_value(images).map_err(Error::from)
    }

    pub fn add_layer(&self, layer: &layer::Layer) -> Result<()> {
        self.inner.addLayer(
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
        let data = source::GeoJsonSourceSpec::new(data)
            .serialize(&ser)
            .map_err(|_| anyhow::anyhow!("Failed to convert GeoJson"))?;

        self.inner.addSource(id.into(), data);

        Ok(())
    }

    pub fn get_geojson_source(&self, id: impl Into<String>) -> Option<source::GeoJsonSource> {
        let source = self.inner.getSource(id.into());

        if !source.is_undefined() {
            Some(GeoJsonSource {
                inner: source.unchecked_into(),
            })
        } else {
            None
        }
    }

    pub fn is_source_loaded(&self, id: impl Into<String>) -> bool {
        self.inner.isSourceLoaded(id.into())
    }

    pub fn are_tiles_loaded(&self) -> bool {
        self.inner.areTilesLoaded()
    }

    pub fn remove_source(&self, id: impl Into<String>) {
        self.inner.removeSource(id.into())
    }

    pub fn pan_to(&self, latlng: LngLat) {
        self.inner
            .panTo(&latlng.inner, JsValue::undefined(), JsValue::undefined());
    }

    pub fn loaded(&self) -> bool {
        self.inner.loaded()
    }

    pub fn get_zoom(&self) -> f64 {
        self.inner.getZoom()
    }
}
