use image::{DynamicImage, ImageFormat};
use wasm_bindgen::prelude::*;
use super::HandleResult;
use js_sys::Reflect;
// 拉伸
pub fn stretch(image: DynamicImage, param: &JsValue, format: ImageFormat) -> HandleResult {

  let w = Reflect::get(param, &"width".into())
    .expect("not found: param [width] field")
    .as_f64()
    .expect("can not convert [width] field to number");

  
  let h = Reflect::get(param, &"height".into())
    .expect("not found: param [height] field")
    .as_f64()
    .expect("can not convert [height] field to number");



  HandleResult {
    image,
    format
  }
}

// 裁切
pub fn splice(image: DynamicImage, param: &JsValue, format: ImageFormat) -> HandleResult {
  
  let w = Reflect::get(param, &"width".into())
    .expect("not found: param [width] field")
    .as_f64()
    .expect("can not convert [width] field to number");

  
  let h = Reflect::get(param, &"height".into())
    .expect("not found: param [height] field")
    .as_f64()
    .expect("can not convert [height] field to number");

  let x = Reflect::get(param, &"x".into())
    .expect("not found: param [x] field")
    .as_f64()
    .expect("can not convert [x] field to number");

  
  let y = Reflect::get(param, &"y".into())
    .expect("not found: param [y] field")
    .as_f64()
    .expect("can not convert [y] field to number");


  HandleResult {
    image,
    format
  }
}