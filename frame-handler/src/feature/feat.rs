use image::{DynamicImage, ImageFormat};
use wasm_bindgen::prelude::*;


pub struct HandleResult {
  pub image: DynamicImage,
  pub format: ImageFormat
}

pub trait Feat {
  fn handle(&self, img: DynamicImage, param: &JsValue, format: ImageFormat) -> HandleResult;
}