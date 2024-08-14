#![doc = include_str!("../README.md")]
mod callback;
pub mod error;
pub mod event;
mod geometry;
pub mod handler;
mod id;
pub mod image;
mod js;
pub mod layer;
pub mod marker;
pub mod popup;
pub mod source;
pub mod style;

use enclose::enclose;
use log::*;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::HashMap,
    ops::DerefMut,
    rc::{Rc, Weak},
};
use wasm_bindgen::{prelude::*, JsCast};

use callback::CallbackStore;
pub use error::{Error, Result};
use geometry::IntoQueryGeometry;
pub use handler::BoxZoomHandler;
pub use id::{CallbackId, MapListenerId, MarkerId};
pub use image::{Image, ImageOptions};
pub use layer::{Layer, Layout, LayoutProperty, Paint};
pub use marker::{Marker, MarkerEventListener, MarkerOptions};
pub use popup::{Popup, PopupOptions};
pub use source::GeoJsonSource;
pub use style::{Source, Sources, Style, StyleOptions, StyleOrRef};

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

impl std::default::Default for LngLat {
    fn default() -> Self {
        LngLat {
            inner: js::LngLat::new(0.0, 0.0),
        }
    }
}

impl std::clone::Clone for LngLat {
    fn clone(&self) -> Self {
        LngLat {
            inner: js::LngLat::new(self.lng(), self.lat()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LngLatValue {
    lng: f64,
    lat: f64,
}

fn serialize_lnglat<S>(lnglat: &js::LngLat, ser: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let value = LngLatValue {
        lng: lnglat.lng(),
        lat: lnglat.lat(),
    };

    value.serialize(ser)
}

fn deserialize_lnglat<'de, D>(de: D) -> std::result::Result<js::LngLat, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = LngLatValue::deserialize(de)?;
    Ok(js::LngLat::new(value.lng, value.lat))
}

fn serialize_lnglat_as_vec<S>(lnglat: &LngLat, ser: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    vec![lnglat.lng(), lnglat.lat()].serialize(ser)
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

impl std::fmt::Display for LngLat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.toString())
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
    style: StyleOrRef,

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
            style: Default::default(),
            projection: None,
            refresh_expired_tiles: None,
            render_world_copies: None,
            scroll_zoom: None,
            test_mode: None,
            zoom: None,
        }
    }

    pub fn style(mut self, style: Style) -> MapOptions {
        self.style = StyleOrRef::Style(style);
        self
    }

    pub fn style_ref(mut self, style: String) -> MapOptions {
        self.style = StyleOrRef::Ref(style);
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
        self.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
            .unwrap()
    }
}
macro_rules! run_macro_with_events {
    ($macro:ident) => {
        $macro! {
            (on_resize, MapBaseEvent);
            (on_remove, MapBaseEvent);

            // Interaction
            (on_mousedown, MapMouseEvent);
            (on_mouseup, MapMouseEvent);
            (on_preclick, MapMouseEvent);
            (on_click, MapMouseEvent);
            (on_dblclick, MapMouseEvent);
            (on_mousemove, MapMouseEvent);
            (on_mouseover, MapMouseEvent);
            (on_mouseenter, MapMouseEvent);
            (on_mouseleave, MapMouseEvent);
            (on_mouseout, MapMouseEvent);
            (on_contextmenu, MapMouseEvent);
            (on_touchstart, MapTouchEvent);
            (on_touchend, MapTouchEvent);
            (on_touchcancel, MapTouchEvent);
            (on_wheel, MapWheelEvent);

            // Movement
            (on_movestart, DragEvent);
            (on_move, MapEvent);
            (on_moveend, DragEvent);
            (on_dragstart, MapBaseEvent);
            (on_drag, DragEvent);
            (on_dragend, DragEvent);
            (on_zoomstart, MapBaseEvent);
            (on_zoom, MapBaseEvent);
            (on_zoomend, MapBaseEvent);
            (on_rotatestart, MapBaseEvent);
            (on_rotate, MapBaseEvent);
            (on_rotateend, MapBaseEvent);
            (on_pitchstart, MapBaseEvent);
            (on_pitch, MapBaseEvent);
            (on_pitchend, MapBaseEvent);
            (on_boxzoomstart, MapBoxZoomEvent);
            (on_boxzoomend, MapBoxZoomEvent);
            (on_boxzoomcancel, MapBoxZoomEvent);

            // Lifecycle
            (on_load, MapBaseEvent);
            (on_render, MapBaseEvent);
            (on_idle, MapBaseEvent);
            (on_webglcontextlost, MapBaseEvent);
            (on_webglcontextrestored, MapBaseEvent);

            // Data loading
            (on_data, MapDataEvent);
            (on_styledata, MapDataEvent);
            (on_sourcedata, MapDataEvent);
            (on_dataloading, MapDataEvent);
            (on_styledataloading, MapDataEvent);
            (on_sourcedataloading, MapBaseEvent);
            (on_styleimagemissing, MapBaseEvent);
        }
    };
}

macro_rules! impl_trait {
    ($(($event:ident, $type:ident);)*) => {

#[allow(unused_variables)]
pub trait MapEventListener {
    $(
        fn $event(&mut self, map: Rc<Map>, e: event::$type) {}
    )*
    fn on_error(&mut self, map: Rc<Map>, _message: String) {}
}

    };
}

macro_rules! define_handle {
    ($(($event:ident, $type:ident);)*) => {

pub struct Handle {
    $(
        $event: Closure<dyn Fn(JsValue)>,
    )*
    on_error: Closure<dyn Fn(JsValue)>,
}

    };
}

macro_rules! impl_handle {
    ($(($event:ident, $type:ident);)*) => {

impl Handle {
    pub fn new<F: MapEventListener + 'static>(map: Weak<Map>, f: F) -> Handle {
        let f = Rc::new(RefCell::new(f));
        Handle {
            $(
                $event: Closure::new(enclose!(
                    (map, f) move |value: JsValue| {
                        let Some(map) = map.upgrade() else {
                            warn!("Failed to get Map handle");
                            return;
                        };

                        match  value.try_into() {
                            Ok(e) => {
                                if let Ok(mut f) = f.try_borrow_mut() {
                                    f.deref_mut().$event(map, e);
                                } else {
                                    error!("Could not borrow {} handler. Handler is being called somewhere?", stringify!($event));
                                }
                            },
                            Err(e) => {
                                error!("Failed to deserialize Event: {e}");
                            }
                        }
                    }
                )),
            )*
            on_error: Closure::new(enclose!(
                (map, f) move |value: JsValue| {
                    web_sys::console::warn_2(&JsValue::from("on_error"), &value);

                    let Some(map) = map.upgrade() else {
                        warn!("Failed to get Map handle");
                        return;
                    };

                    if let Ok(mut f) = f.try_borrow_mut() {
                        f.deref_mut().on_error(map, value.as_string().unwrap());
                    } else {
                        error!("Could not borrow on_error handler. Handler is being called somewhere?");
                    }
                }
            )),
        }
    }
}

    };
}

run_macro_with_events!(impl_trait);
run_macro_with_events!(define_handle);
run_macro_with_events!(impl_handle);

macro_rules! impl_on_method {
    ($(($event:ident, $type:ident);)*) => {

/// Add a listener to a specified event type.
pub fn on<F: MapEventListener + 'static>(&self, f: F) -> Result<MapListenerId> {
    let handle = Handle::new(
        self.weak_self
            .try_borrow()
            .map_err(|e| Error::Unexpected(format!("Could not borrow weak_self: {e}")))?
            .clone()
            .ok_or_else(|| Error::Unexpected("Weak reference is missing".to_string()))?,
        f,
    );
    let inner = &self.inner;

    $(
        inner.on(stringify!($event).trim_start_matches("on_").into(), &handle.$event);
    )*

    inner.on("error".into(), &handle.on_error);

    let id = MapListenerId(uuid::Uuid::new_v4());
    self.handles
        .try_borrow_mut()
        .map_err(|e| Error::Unexpected(format!("Could not get lock for handles: {e}")))?
        .insert(id, handle);

    Ok(id)
}

    };
}

macro_rules! impl_on_layer_method {
    ($(($event:ident, $type:ident);)*) => {

/// Add a listener to a specified event type and layer.
pub fn on_layer<F: MapEventListener + 'static>(&self, layer_id: &str, f: F) -> Result<MapListenerId> {
    let handle = Handle::new(
        self.weak_self
            .try_borrow()
            .map_err(|e| Error::Unexpected(format!("Could not borrow weak_self: {e}")))?
            .clone()
            .ok_or_else(|| Error::Unexpected("Weak reference is missing".to_string()))?,
        f,
    );
    let inner = &self.inner;

    $(
        inner.on_layer(stringify!($event).trim_start_matches("on_").into(), layer_id.into(), &handle.$event);
    )*

    inner.on_layer("error".into(), layer_id.into(), &handle.on_error);

    let id = MapListenerId(uuid::Uuid::new_v4());
    self.handles
        .try_borrow_mut()
        .map_err(|e| Error::Unexpected(format!("Could not get lock for handles: {e}")))?
        .insert(id, handle);

    Ok(id)
}

    };
}

pub struct Map {
    pub(crate) inner: crate::js::Map,
    pub(crate) handles: RefCell<HashMap<MapListenerId, Handle>>,
    pub(crate) markers: RefCell<HashMap<MarkerId, Rc<Marker>>>,
    pub(crate) image_cbs: CallbackStore<dyn FnMut(JsValue, JsValue) + 'static>,
    pub(crate) weak_self: RefCell<Option<Weak<Map>>>,
}

impl Map {
    pub fn new(options: MapOptions) -> Result<Rc<Map>> {
        let options = options.build();

        let inner = crate::js::Map::new(options);

        let map = Rc::new(Map {
            inner,
            handles: RefCell::new(HashMap::new()),
            markers: RefCell::new(HashMap::new()),
            image_cbs: CallbackStore::new(),
            weak_self: RefCell::new(None),
        });

        let weak_self = Rc::downgrade(&map);

        *map.weak_self
            .try_borrow_mut()
            .map_err(|e| Error::Unexpected(e.to_string()))? = Some(weak_self);

        Ok(map)
    }

    run_macro_with_events!(impl_on_method);
    run_macro_with_events!(impl_on_layer_method);

    pub fn add_marker(&self, marker: Rc<Marker>) -> MarkerId {
        let id = MarkerId(uuid::Uuid::new_v4());
        marker.add_to(self);
        self.markers
            .try_borrow_mut()
            .expect("Could not get lock for markders")
            .insert(id, marker);

        id
    }

    pub fn remove_marker(&self, id: &MarkerId) {
        self.markers
            .try_borrow_mut()
            .expect("Could not get lock for markders")
            .get(id)
            .unwrap()
            .remove();

        self.markers
            .try_borrow_mut()
            .expect("Could not get lock for markders")
            .remove(id);
    }
}

#[derive(Debug, Clone)]
enum HandlerType {
    BoxZoom,
}

impl std::fmt::Display for HandlerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandlerType::BoxZoom => write!(f, "boxZoom"),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct QueryFeatureOptions {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CameraOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub around: Option<LngLat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<f64>,
    #[serde(serialize_with = "serialize_lnglat_as_vec")]
    pub center: LngLat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<PaddingOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaddingOptions {
    pub bottom: Option<LngLat>,
    pub left: Option<LngLat>,
    pub right: Option<LngLat>,
    pub top: Option<LngLat>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    // TODO
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub easing: Option<xxx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_zoom: Option<f64>,
    // TODO
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub offset: Option<xxx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preloading_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_speed: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CameraAnimationOptions {
    #[serde(flatten)]
    camera_options: CameraOptions,
    #[serde(flatten)]
    animation_options: AnimationOptions,
}

impl Map {
    pub fn get_container(&self) -> web_sys::HtmlElement {
        self.inner.getContainer()
    }

    pub fn get_bounds(&self) -> Result<LngLatBounds> {
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

    pub fn set_style(&self, style: Style, options: StyleOptions) {
        self.inner.setStyle(
            style.into(),
            serde_wasm_bindgen::to_value(&options).unwrap(),
        );
    }

    pub fn set_style_ref(&self, style: impl Into<String>, options: StyleOptions) {
        self.inner.setStyle(
            style.into().into(),
            serde_wasm_bindgen::to_value(&options).unwrap(),
        );
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
        let url = url.into();
        let callback_id = CallbackId(uuid::Uuid::new_v4());
        let cbs = self.image_cbs.clone();

        let cloned_url = url.clone();
        let callback = Closure::new(move |e: JsValue, image| {
            if e.is_null() {
                callback(Ok(crate::image::Image { inner: image }));
            } else {
                callback(Err(Error::LoadImage(cloned_url.clone())));
            }

            if let Err(e) = cbs.remove(&callback_id) {
                warn!("{e}");
            }
        });

        self.inner.loadImage(url, &callback);

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
        self.inner
            .addLayer(serde_wasm_bindgen::to_value(&layer).map_err(Error::from)?);

        Ok(())
    }

    pub fn query_rendered_features<G: IntoQueryGeometry>(
        &self,
        geometry: Option<G>,
        options: QueryFeatureOptions,
    ) -> Result<Vec<geojson::Feature>> {
        // It seems GeoJSON returned from mapbox-gl-js queryRenderedFeatures contains
        // byte array, which causes deserialize error with geojson crate. geojson
        // crate internally deserialize all the properties into Map.
        //
        // As a workaround, use an intermediate "Feature" struct to deserialize from
        // JsValue using serde, then convert it to geojson::Feature.
        #[derive(Debug, serde::Deserialize)]
        #[serde(untagged)]
        enum Id {
            String(String),
            Number(serde_json::Number),
        }

        #[derive(Debug, serde::Deserialize)]
        struct Feature {
            bbox: Option<geojson::Bbox>,
            geometry: Option<geojson::Geometry>,
            id: Option<Id>,
            properties: Option<geojson::JsonObject>,
            foreign_members: Option<geojson::JsonObject>,
        }
        let res = self.inner.queryRenderedFeatures(
            serde_wasm_bindgen::to_value(&geometry.map(|g| g.into_query_geometry().into_vec()))?,
            serde_wasm_bindgen::to_value(&options)?,
        );

        let features: Vec<Feature> = serde_wasm_bindgen::from_value(res)?;

        features
            .into_iter()
            .map(|f| {
                Ok(geojson::Feature {
                    bbox: f.bbox,
                    geometry: f.geometry,
                    id: f.id.map(|id| match id {
                        Id::String(s) => geojson::feature::Id::String(s),
                        Id::Number(n) => geojson::feature::Id::Number(n),
                    }),
                    properties: f.properties,
                    foreign_members: f.foreign_members,
                })
            })
            .collect()
    }

    pub fn add_vector_source(&self, id: impl Into<String>, url: impl Into<String>) -> Result<()> {
        #[derive(Serialize)]
        struct VectorSource {
            r#type: String,
            url: String,
        }

        self.inner.addSource(
            id.into(),
            serde_wasm_bindgen::to_value(&VectorSource {
                r#type: "vector".into(),
                url: url.into(),
            })?,
        );

        Ok(())
    }

    pub fn add_geojson_source(&self, id: impl Into<String>, data: geojson::GeoJson) -> Result<()> {
        let ser = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        let data = source::GeoJsonSourceSpec::new(data)
            .serialize(&ser)
            .map_err(|e| Error::BadGeoJson(e.to_string()))?;

        self.inner.addSource(id.into(), data);

        Ok(())
    }

    pub fn add_geojson_source_from_url(
        &self,
        id: impl Into<String>,
        data: impl Into<String>,
    ) -> Result<()> {
        let ser = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        let data = source::GeoJsonSourceSpec::new(data.into())
            .serialize(&ser)
            .map_err(|e| Error::BadGeoJson(e.to_string()))?;

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

    pub fn jump_to(&self, options: CameraOptions) {
        self.inner
            .jumpTo(serde_wasm_bindgen::to_value(&options).unwrap());
    }

    pub fn ease_to(&self, camera_options: CameraOptions, animation_options: AnimationOptions) {
        // This serializer is always needed when using serde's 'flatten' directive to js
        let ser = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        let options = CameraAnimationOptions {
            camera_options,
            animation_options,
        };

        self.inner.easeTo(options.serialize(&ser).unwrap());
    }

    pub fn fly_to(&self, camera_options: CameraOptions, animation_options: AnimationOptions) {
        // This serializer is always needed when using serde's 'flatten' directive to js
        let ser = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        let options = CameraAnimationOptions {
            camera_options,
            animation_options,
        };

        self.inner
            .flyTo(options.serialize(&ser).unwrap(), JsValue::null());
    }
}
