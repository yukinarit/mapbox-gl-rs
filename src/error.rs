pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to load image {0}")]
    LoadImage(String),
    #[error("Bad GeoJSON: {0}")]
    BadGeoJson(String),
    #[error("The object is not compatible to {0}: {1}")]
    BadEventFormat(&'static str, String),
    #[error("Error: {0}")]
    Unexpected(String),
    /// Error from Js/Rust conversions
    #[error("JsError: {0}")]
    JsError(String),
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Error::JsError(e.to_string())
    }
}
