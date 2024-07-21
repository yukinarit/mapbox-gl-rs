// Because of https://github.com/rustwasm/wasm-bindgen/issues/3945 we need to allow this lint for now
#![allow(clippy::empty_docs)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type LngLat;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    pub fn new(lng: f64, lat: f64) -> LngLat;

    #[wasm_bindgen(method, getter)]
    pub fn lng(this: &LngLat) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_lng(this: &LngLat, v: f64) -> LngLat;

    #[wasm_bindgen(method, getter)]
    pub fn lat(this: &LngLat) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_lat(this: &LngLat, v: f64) -> LngLat;

    #[wasm_bindgen(method)]
    pub fn wrap(this: &LngLat) -> LngLat;

    #[wasm_bindgen(method)]
    pub fn toArray(this: &LngLat) -> Vec<f64>;

    #[wasm_bindgen(method)]
    pub fn toString(this: &LngLat) -> String;

    #[wasm_bindgen(method)]
    pub fn distanceTo(this: &LngLat, lngLat: &LngLat) -> f64;

    #[wasm_bindgen(method)]
    pub fn toBounds(this: &LngLat, radius: f64) -> LngLatBounds;

    // --

    pub type LngLatBounds;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    pub fn new(sw: LngLat, ne: LngLat) -> LngLatBounds;

    #[wasm_bindgen(method)]
    pub fn setNorthEast(this: &LngLatBounds, ne: LngLat) -> LngLat;

    #[wasm_bindgen(method)]
    pub fn setSouthWest(this: &LngLatBounds, sw: LngLat) -> LngLat;

    #[wasm_bindgen(method)]
    pub fn getCenter(this: &LngLatBounds) -> LngLat;

    #[wasm_bindgen(method)]
    pub fn getSouthWest(this: &LngLatBounds) -> LngLat;

    #[wasm_bindgen(method)]
    pub fn getNorthEast(this: &LngLatBounds) -> LngLat;

    #[wasm_bindgen(method)]
    pub fn getNorthWest(this: &LngLatBounds) -> LngLat;

    #[wasm_bindgen(method)]
    pub fn getSouthEast(this: &LngLatBounds) -> LngLat;

    #[wasm_bindgen(method)]
    pub fn getWest(this: &LngLatBounds) -> f64;

    #[wasm_bindgen(method)]
    pub fn getSouth(this: &LngLatBounds) -> f64;

    #[wasm_bindgen(method)]
    pub fn getEast(this: &LngLatBounds) -> f64;

    #[wasm_bindgen(method)]
    pub fn getNorth(this: &LngLatBounds) -> f64;

    #[wasm_bindgen(method)]
    pub fn isEmpty(this: &LngLatBounds) -> f64;

    #[wasm_bindgen(method)]
    pub fn contains(this: &LngLat) -> bool;

    #[wasm_bindgen(method)]
    pub fn toString(this: &LngLatBounds) -> String;

    // --

    pub type Map;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    pub fn new(options: JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn getBounds(this: &Map) -> LngLatBounds;

    #[wasm_bindgen(method)]
    pub fn getZoom(this: &Map) -> f64;

    #[wasm_bindgen(method)]
    pub fn jumpTo(this: &Map, options: JsValue);

    #[wasm_bindgen(method)]
    pub fn easeTo(this: &Map, options: JsValue);

    #[wasm_bindgen(method)]
    pub fn flyTo(this: &Map, options: JsValue, eventData: JsValue);

    #[wasm_bindgen(method)]
    pub fn on(this: &Map, r#type: String, callback: &Closure<dyn Fn(JsValue)>);

    #[wasm_bindgen(method, js_name=on)]
    pub fn on_layer(
        this: &Map,
        r#type: String,
        layer_id: String,
        callback: &Closure<dyn Fn(JsValue)>,
    );

    #[wasm_bindgen(method)]
    pub fn getContainer(this: &Map) -> web_sys::HtmlElement;

    #[wasm_bindgen(method)]
    pub fn getMinZoom(this: &Map) -> f64;

    #[wasm_bindgen(method)]
    pub fn isMoving(this: &Map) -> bool;

    #[wasm_bindgen(method)]
    pub fn isZooming(this: &Map) -> bool;

    #[wasm_bindgen(method)]
    pub fn isRotating(this: &Map) -> bool;

    #[wasm_bindgen(method)]
    pub fn setStyle(this: &Map, style: JsValue, options: JsValue) -> bool;

    // Images

    /// Add image resource.
    /// https://docs.mapbox.com/mapbox-gl-js/api/map/#map#addimage
    #[wasm_bindgen(method)]
    pub fn addImage(this: &Map, id: String, image: JsValue, options: JsValue);

    /// Update an existing image in a style.
    /// https://docs.mapbox.com/mapbox-gl-js/api/map/#map#loadimage
    #[wasm_bindgen(method)]
    pub fn updateImage(this: &Map, id: String, image: JsValue) -> bool;

    /// Check whether or not an image with a specific ID exists in the style.
    /// https://docs.mapbox.com/mapbox-gl-js/api/map/#map#updateimage
    #[wasm_bindgen(method)]
    pub fn hasImage(this: &Map, id: String) -> bool;

    /// Remove an image from a style.
    /// https://docs.mapbox.com/mapbox-gl-js/api/map/#map#removeimage
    #[wasm_bindgen(method)]
    pub fn removeImage(this: &Map, id: String);

    /// Load an image from an external URL.
    /// https://docs.mapbox.com/mapbox-gl-js/api/map/#map#loadimage
    #[wasm_bindgen(method)]
    pub fn loadImage(this: &Map, url: String, callback: &Closure<dyn FnMut(JsValue, JsValue)>);

    /// Returns an Array of strings containing the IDs of all images currently available in the
    /// map.
    /// https://docs.mapbox.com/mapbox-gl-js/api/map/#map#listimages
    #[wasm_bindgen(method)]
    pub fn listImages(this: &Map) -> JsValue;

    // Lifecycle
    #[wasm_bindgen(method)]
    pub fn loaded(this: &Map) -> bool;

    #[wasm_bindgen(method)]
    pub fn remove(this: &Map);

    #[wasm_bindgen(method)]
    pub fn triggerRepaint(this: &Map);

    #[wasm_bindgen(method)]
    pub fn queryRenderedFeatures(map: &Map, geometry: JsValue, options: JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn addSource(this: &Map, id: String, source: JsValue);

    #[wasm_bindgen(method)]
    pub fn isSourceLoaded(this: &Map, id: String) -> bool;

    #[wasm_bindgen(method)]
    pub fn areTilesLoaded(this: &Map) -> bool;

    #[wasm_bindgen(method)]
    pub fn removeSource(this: &Map, id: String);

    #[wasm_bindgen(method)]
    pub fn getSource(this: &Map, id: String) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn addLayer(this: &Map, value: JsValue);

    #[wasm_bindgen(method, setter)]
    pub fn set_showTileBoundaries(this: &Map, v: bool) -> Map;

    #[wasm_bindgen(method, getter)]
    pub fn showTileBoundaries(this: &Map) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_showTerrainWireframe(this: &Map, v: bool) -> Map;

    #[wasm_bindgen(method, getter)]
    pub fn showTerrainWireframe(this: &Map) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_showPadding(this: &Map, v: bool) -> Map;

    #[wasm_bindgen(method, getter)]
    pub fn showPadding(this: &Map) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_showCollisionBoxes(this: &Map, v: bool) -> Map;

    #[wasm_bindgen(method, getter)]
    pub fn showCollisionBoxes(this: &Map) -> bool;

    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn get_handler(this: &Map, name: &str) -> JsValue;

    #[wasm_bindgen(method, structural, indexing_setter)]
    pub fn set_handler(this: &Map, name: &str, handler: BoxZoomHandler) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn panTo(this: &Map, lngLat: &LngLat, options: JsValue, eventData: JsValue);

    // Camera
    #[wasm_bindgen(method)]
    pub fn getCenter(this: &Map) -> LngLat;

    // --

    pub type BoxZoomHandler;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    pub fn BoxZoomHandler_new(map: &Map, options: JsValue) -> BoxZoomHandler;

    #[wasm_bindgen(method, js_name=enable)]
    pub fn BoxZoomHandler_enable(this: &BoxZoomHandler);

    #[wasm_bindgen(method, js_name=disable)]
    pub fn BoxZoomHandler_disable(this: &BoxZoomHandler);

    #[wasm_bindgen(method, js_name=isEnabled)]
    pub fn BoxZoomHandler_isEnabled(this: &BoxZoomHandler) -> bool;

    #[wasm_bindgen(method, js_name=isActive)]
    pub fn BoxZoomHandler_isActive(this: &BoxZoomHandler) -> bool;

    #[wasm_bindgen(method, js_name=disableRotation)]
    pub fn BoxZoomHandler_disableRotation(this: &BoxZoomHandler);

    #[wasm_bindgen(method, js_name=enableRotation)]
    pub fn BoxZoomHandler_enableRotation(this: &BoxZoomHandler);

    // --

    pub type Marker;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    pub fn maker_new(options: JsValue) -> Marker;

    #[wasm_bindgen(method)]
    pub fn setLngLat(this: &Marker, lngLat: &LngLat);

    #[wasm_bindgen(method)]
    pub fn addTo(this: &Marker, map: &Map);

    #[wasm_bindgen(method)]
    pub fn remove(this: &Marker);

    #[wasm_bindgen(method, js_name=on)]
    pub fn Marker_on(this: &Marker, r#type: String, callback: &Closure<dyn Fn(JsValue)>);

    #[wasm_bindgen(method)]
    pub fn getLngLat(this: &Marker) -> LngLat;

    // --

    pub type Popup;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    pub fn Popup_new(options: JsValue) -> Popup;

    #[wasm_bindgen(method, js_name=setHTML)]
    pub fn Popup_setHTML(this: &Popup, html: String);

    #[wasm_bindgen(method, js_name=setLngLat)]
    pub fn Popup_setLngLat(this: &Popup, lngLat: &LngLat);

    #[wasm_bindgen(method, js_name=addTo)]
    pub fn Popup_addTo(this: &Popup, map: &Map);

    // --

    pub type GeoJSONSource;

    #[wasm_bindgen(method, js_name=setData)]
    pub fn GeoJSONSource_setData(this: &GeoJSONSource, data: &JsValue);
}
