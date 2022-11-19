use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type LngLatLike;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    pub fn new(lng: f64, lat: f64) -> LngLatLike;

    #[wasm_bindgen(method, getter)]
    pub fn lng(this: &LngLatLike) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_lng(this: &LngLatLike, v: f64) -> LngLatLike;

    #[wasm_bindgen(method, getter)]
    pub fn lat(this: &LngLatLike) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_lat(this: &LngLatLike, v: f64) -> LngLatLike;

    pub type LngLatBounds;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    pub fn new(sw: LngLatLike, ne: LngLatLike) -> LngLatBounds;

    #[wasm_bindgen(method, getter)]
    pub fn sw(this: &LngLatBounds) -> LngLatLike;

    #[wasm_bindgen(method, getter)]
    pub fn ne(this: &LngLatBounds) -> LngLatLike;

    ///
    /// Map
    ///
    pub type Map;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    pub fn new(options: JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn getBounds(this: &Map) -> LngLatBounds;

    #[wasm_bindgen(method)]
    pub fn getZoom(this: &Map) -> f64;

    #[wasm_bindgen(method)]
    pub fn on(this: &Map, r#type: String, callback: &Closure<dyn Fn(JsValue)>);

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

    // Lifecycle
    #[wasm_bindgen(method, js_name=loaded)]
    pub fn Map_loaded(this: &Map) -> bool;

    #[wasm_bindgen(method)]
    pub fn remove(this: &Map);

    #[wasm_bindgen(method)]
    pub fn triggerRepaint(this: &Map);

    #[wasm_bindgen(method, js_name=addSource)]
    pub fn Map_addSource(this: &Map, id: String, value: JsValue);

    #[wasm_bindgen(method, js_name=getSource)]
    pub fn Map_getSource(this: &Map, id: String) -> JsValue;

    #[wasm_bindgen(method, js_name=addLayer)]
    pub fn Map_addLayer(this: &Map, value: JsValue);

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

    #[wasm_bindgen(method, js_name=panTo)]
    pub fn Map_panTo(this: &Map, lngLat: crate::LatLng, options: JsValue, eventData: JsValue);

    ///
    /// Handler
    ///
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

    ///
    /// Marker
    ///
    pub type Marker;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    pub fn maker_new(options: JsValue) -> Marker;

    #[wasm_bindgen(method)]
    pub fn setLngLat(this: &Marker, lngLat: crate::LatLng);

    #[wasm_bindgen(method)]
    pub fn addTo(this: &Marker, map: &Map);

    #[wasm_bindgen(method)]
    pub fn remove(this: &Marker);

    ///
    /// Popup
    ///
    pub type Popup;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    pub fn Popup_new(options: JsValue) -> Popup;

    #[wasm_bindgen(method, js_name=setHTML)]
    pub fn Popup_setHTML(this: &Popup, html: String);

    #[wasm_bindgen(method, js_name=setLngLat)]
    pub fn Popup_setLngLat(this: &Popup, lngLat: crate::LatLng);

    #[wasm_bindgen(method, js_name=addTo)]
    pub fn Popup_addTo(this: &Popup, map: &Map);

    ///
    /// Popup
    ///
    pub type GeoJSONSource;

    #[wasm_bindgen(method, js_name=setData)]
    pub fn GeoJSONSource_setData(this: &GeoJSONSource, data: &JsValue);
}
