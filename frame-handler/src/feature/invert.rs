use image::DynamicImage;
use wasm_bindgen::prelude::*;
use super::feat::Feat;

pub struct Invert {}

impl Feat for Invert {
    fn handle (&self, mut img: DynamicImage, _param: &JsValue) -> DynamicImage {
        img.invert();
        img
    } 
}