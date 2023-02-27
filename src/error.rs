use wasm_bindgen::prelude::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to load image: {0:?}")]
    LoadImage(JsValue),
    #[error("Error: {0}")]
    Unexpected(String),
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Error::Unexpected(e.to_string())
    }
}
