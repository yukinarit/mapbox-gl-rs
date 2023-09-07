use anyhow::anyhow;
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
    type Error = anyhow::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let r#type = js_sys::Reflect::get(&value, &JsValue::from("type"))
            .unwrap()
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
    type Error = anyhow::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let r#type = js_sys::Reflect::get(&value, &JsValue::from("type"))
            .unwrap()
            .as_string()
            .unwrap();
        let original_event = js_sys::Reflect::get(&value, &JsValue::from("originalEvent"))
            .ok()
            .and_then(|event| web_sys::MouseEvent::try_from(event).ok());
        Ok(MapEvent {
            r#type,
            original_event,
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
    type Error = anyhow::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(value)
            .map_err(|e| anyhow!("Failed to deserialize into \"MapDataEvent\": {}", e))
    }
}

#[derive(Debug, Clone)]
pub struct MapBoxZoomEvent {
    pub r#type: String,
    pub original_event: web_sys::MouseEvent,
}

impl TryFrom<JsValue> for MapBoxZoomEvent {
    type Error = anyhow::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let r#type = js_sys::Reflect::get(&value, &JsValue::from("type"))
            .map_err(|_| anyhow!("\"type\" property not found"))?
            .as_string()
            .ok_or_else(|| anyhow!("Failed to cast \"type\" property as string"))?;

        let event = js_sys::Reflect::get(&value, &JsValue::from("originalEvent"))
            .map_err(|_| anyhow!("\"originalEvent\" property not found"))?;

        Ok(MapBoxZoomEvent {
            r#type,
            original_event: web_sys::MouseEvent::try_from(event)?,
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
    type Error = anyhow::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let r#type = js_sys::Reflect::get(&value, &JsValue::from("type"))
            .map_err(|_| anyhow!("\"type\" property not found"))?
            .as_string()
            .ok_or_else(|| anyhow!("Failed to cast \"type\" property as string"))?;

        let event = js_sys::Reflect::get(&value, &JsValue::from("originalEvent"))
            .map_err(|_| anyhow!("\"originalEvent\" property not found"))?;

        let point = js_sys::Reflect::get(&value, &JsValue::from("point"))
            .map_err(|_| anyhow!("\"point\" property not found"))?;

        let lng_lat = js_sys::Reflect::get(&value, &JsValue::from("lngLat"))
            .map_err(|_| anyhow!("\"lngLat\" property not found"))?;

        let features = js_sys::Reflect::get(&value, &JsValue::from("features"))
            .map_err(|_| anyhow!("\"features\" property not found"))?;

        Ok(MapMouseEvent {
            r#type,
            original_event: web_sys::MouseEvent::try_from(event)?,
            point: serde_wasm_bindgen::from_value(point).unwrap(),
            lng_lat: serde_wasm_bindgen::from_value(lng_lat).unwrap(),
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
    type Error = anyhow::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let r#type = js_sys::Reflect::get(&value, &JsValue::from("type"))
            .map_err(|_| anyhow!("\"type\" property not found"))?
            .as_string()
            .ok_or_else(|| anyhow!("Failed to cast \"type\" property as string"))?;

        let event = js_sys::Reflect::get(&value, &JsValue::from("originalEvent"))
            .map_err(|_| anyhow!("\"originalEvent\" property not found"))?;

        let point = js_sys::Reflect::get(&value, &JsValue::from("point"))
            .map_err(|_| anyhow!("\"point\" property not found"))?;

        let points = js_sys::Reflect::get(&value, &JsValue::from("points"))
            .map_err(|_| anyhow!("\"points\" property not found"))?;

        let lng_lat = js_sys::Reflect::get(&value, &JsValue::from("lngLat"))
            .map_err(|_| anyhow!("\"lngLat\" property not found"))?;

        let lng_lats = js_sys::Reflect::get(&value, &JsValue::from("lngLats"))
            .map_err(|_| anyhow!("\"lngLats\" property not found"))?;

        let features = js_sys::Reflect::get(&value, &JsValue::from("features"))
            .map_err(|_| anyhow!("\"features\" property not found"))?;

        Ok(MapTouchEvent {
            r#type,
            original_event: web_sys::TouchEvent::try_from(event)?,
            point: serde_wasm_bindgen::from_value(point).unwrap(),
            points: serde_wasm_bindgen::from_value(points).unwrap(),
            lng_lat: serde_wasm_bindgen::from_value(lng_lat).unwrap(),
            lng_lats: serde_wasm_bindgen::from_value(lng_lats).unwrap(),
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
    type Error = anyhow::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let r#type = js_sys::Reflect::get(&value, &JsValue::from("type"))
            .map_err(|_| anyhow!("\"type\" property not found"))?
            .as_string()
            .ok_or_else(|| anyhow!("Failed to cast \"type\" property as string"))?;

        let event = js_sys::Reflect::get(&value, &JsValue::from("originalEvent"))
            .map_err(|_| anyhow!("\"originalEvent\" property not found"))?;

        Ok(MapWheelEvent {
            r#type,
            original_event: web_sys::WheelEvent::try_from(event)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DragEvent {
    pub original_event: web_sys::DragEvent,
}

impl TryFrom<JsValue> for DragEvent {
    type Error = anyhow::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let event = js_sys::Reflect::get(&value, &JsValue::from("originalEvent"))
            .map_err(|_| anyhow!("\"originalEvent\" property not found"))?;

        Ok(DragEvent {
            original_event: web_sys::DragEvent::try_from(event)?,
        })
    }
}
