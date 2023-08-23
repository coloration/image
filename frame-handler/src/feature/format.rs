use image::DynamicImage;
use wasm_bindgen::prelude::*;
use super::feat::Feat;

pub struct Format {}

impl Feat for Format {
    fn handle (&self, img: DynamicImage, _param: &JsValue) -> DynamicImage {
        img
    }
}