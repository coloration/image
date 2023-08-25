use image::{DynamicImage, ImageFormat};
use wasm_bindgen::prelude::*;
use super::{feat::Feat, HandleResult};

pub struct Grayscale {}

impl Feat for Grayscale {
    fn handle (&self, image: DynamicImage, _param: &JsValue, format: ImageFormat) -> HandleResult {
        
        HandleResult {
            image: image.grayscale(),
            format
        }
    }
}
