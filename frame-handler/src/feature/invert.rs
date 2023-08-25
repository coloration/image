use image::{DynamicImage, ImageFormat};
use wasm_bindgen::prelude::*;
use super::{feat::Feat, HandleResult};

pub struct Invert {}

impl Feat for Invert {
    fn handle (&self, mut image: DynamicImage, _param: &JsValue, format: ImageFormat) -> HandleResult {
        image.invert();
        
        HandleResult { image, format }
    } 
}