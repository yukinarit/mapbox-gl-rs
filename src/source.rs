use crate::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GeoJsonSourceSpec<S: Serialize> {
    pub r#type: String,
    pub data: S,
}

impl<S> GeoJsonSourceSpec<S>
where
    S: Serialize,
{
    pub fn new(data: S) -> GeoJsonSourceSpec<S> {
        GeoJsonSourceSpec {
            r#type: "geojson".into(),
            data,
        }
    }
}

pub struct GeoJsonSource {
    pub inner: crate::js::GeoJSONSource,
}

impl GeoJsonSource {
    pub fn set_data(&mut self, data: geojson::GeoJson) -> Result<()> {
        let ser = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);

        self.inner.GeoJSONSource_setData(&data.serialize(&ser)?);

        Ok(())
    }
    pub fn set_data_js_value(&mut self, data: &wasm_bindgen::JsValue) -> Result<()> {
        self.inner.GeoJSONSource_setData(data);
        Ok(())
    }
}
