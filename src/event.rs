use crate::{Error, Result};
use log::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LngLat {
    pub lng: f64,
    pub lat: f64,
}

#[derive(Debug, Clone)]
pub struct MapBaseEvent {
    pub r#type: String,
}

impl TryFrom<JsValue> for MapBaseEvent {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let r#type = get_property(&value, "MapBaseEvent", "type")?
            .as_string()
            .unwrap();

        Ok(MapBaseEvent { r#type })
    }
}

#[derive(Debug, Clone)]
pub struct MapEvent {
    pub r#type: String,
    pub original_event: Option<web_sys::MouseEvent>,
}

impl TryFrom<JsValue> for MapEvent {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let r#type = get_property(&value, "MapEvent", "type")?
            .as_string()
            .unwrap();
        let event = get_property(&value, "MapEvent", "originalEvent")?;

        Ok(MapEvent {
            r#type,
            original_event: web_sys::MouseEvent::try_from(event).ok(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapDataEvent {
    pub r#type: String,
    pub data_type: String,
    pub is_source_loaded: Option<bool>,
    // TODO source
    pub source_data_type: Option<String>,
    pub source_id: Option<String>,
    // TODO tile
}

impl TryFrom<JsValue> for MapDataEvent {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        Ok(serde_wasm_bindgen::from_value(value)?)
    }
}

#[derive(Debug, Clone)]
pub struct MapBoxZoomEvent {
    pub r#type: String,
    pub original_event: web_sys::MouseEvent,
}

impl TryFrom<JsValue> for MapBoxZoomEvent {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let r#type = get_property(&value, "MapBoxZoomEvent", "type")?
            .as_string()
            .unwrap();
        let event = get_property(&value, "MapBoxZoomEvent", "originalEvent")?;

        Ok(MapBoxZoomEvent {
            r#type,
            original_event: web_sys::MouseEvent::try_from(event).unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MapMouseEvent {
    pub r#type: String,
    pub original_event: web_sys::MouseEvent,
    pub point: Point,
    pub lng_lat: LngLat,
    pub features: Vec<String>,
}

impl TryFrom<JsValue> for MapMouseEvent {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let r#type = get_property(&value, "MapMouseEvent", "type")?
            .as_string()
            .unwrap();
        let event = get_property(&value, "MapMouseEvent", "originalEvent")?;
        let point = get_property(&value, "MapMouseEvent", "point")?;
        let lng_lat = get_property(&value, "MapMouseEvent", "lngLat")?;
        let features = get_property(&value, "MapMouseEvent", "features")?;

        Ok(MapMouseEvent {
            r#type,
            original_event: web_sys::MouseEvent::try_from(event).unwrap(),
            point: serde_wasm_bindgen::from_value(point)?,
            lng_lat: serde_wasm_bindgen::from_value(lng_lat)?,
            features: serde_wasm_bindgen::from_value(features).unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MapTouchEvent {
    pub r#type: String,
    pub original_event: web_sys::TouchEvent,
    pub point: Point,
    pub points: Vec<Point>,
    pub lng_lat: LngLat,
    pub lng_lats: Vec<LngLat>,
    pub features: Vec<String>,
}

impl TryFrom<JsValue> for MapTouchEvent {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let r#type = get_property(&value, "MapTouchEvent", "type")?
            .as_string()
            .unwrap();
        let event = get_property(&value, "MapTouchEvent", "originalEvent")?;
        let point = get_property(&value, "MapTouchEvent", "point")?;
        let points = get_property(&value, "MapTouchEvent", "points")?;
        let lng_lat = get_property(&value, "MapTouchEvent", "lngLat")?;
        let lng_lats = get_property(&value, "MapTouchEvent", "lngLats")?;
        let features = get_property(&value, "MapTouchEvent", "features")?;

        Ok(MapTouchEvent {
            r#type,
            original_event: web_sys::TouchEvent::try_from(event).unwrap(),
            point: serde_wasm_bindgen::from_value(point)?,
            points: serde_wasm_bindgen::from_value(points)?,
            lng_lat: serde_wasm_bindgen::from_value(lng_lat)?,
            lng_lats: serde_wasm_bindgen::from_value(lng_lats)?,
            features: serde_wasm_bindgen::from_value(features).unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MapWheelEvent {
    pub r#type: String,
    pub original_event: web_sys::WheelEvent,
}

impl TryFrom<JsValue> for MapWheelEvent {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let r#type = get_property(&value, "MapWheelEvent", "type")?
            .as_string()
            .unwrap();
        let event = get_property(&value, "MapWheelEvent", "originalEvent")?;

        Ok(MapWheelEvent {
            r#type,
            original_event: web_sys::WheelEvent::try_from(event).unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DragEvent {
    pub original_event: web_sys::DragEvent,
}

impl TryFrom<JsValue> for DragEvent {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self> {
        let event = get_property(&value, "DragEvent", "originalEvent")?;

        Ok(DragEvent {
            original_event: web_sys::DragEvent::try_from(event).unwrap(),
        })
    }
}

fn get_property(
    value: &JsValue,
    event_name: &'static str,
    property_name: &'static str,
) -> Result<JsValue> {
    trace!("Getting {property_name} on {value:?}");
    js_sys::Reflect::get(value, &JsValue::from(property_name)).map_err(|_| {
        Error::BadEventFormat(
            event_name,
            format!("property \"{property_name}\" is missing"),
        )
    })
}
