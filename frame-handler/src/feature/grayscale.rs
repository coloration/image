use image::DynamicImage;
use wasm_bindgen::prelude::*;
use super::feat::Feat;

pub struct Grayscale {}

impl Feat for Grayscale {
    fn handle (&self, img: DynamicImage, _param: &JsValue) -> DynamicImage {
        img.grayscale()
    }
}
