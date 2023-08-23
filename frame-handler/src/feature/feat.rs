use image::DynamicImage;
use wasm_bindgen::prelude::*;


pub trait Feat {
  fn handle(&self, img: DynamicImage, param: &JsValue) -> DynamicImage;
}