use image::{DynamicImage, ImageFormat, imageops::FilterType};
use wasm_bindgen::prelude::*;
use crate::utils::*;

use super::{feat::Feat, HandleResult};
// 拉伸
pub struct Resize {}

impl Feat for Resize {
  fn handle(&self, image: DynamicImage, param: &JsValue, format: ImageFormat) -> HandleResult {

    let w = param_field_number(param, "width") as u32;
    let h = param_field_number(param, "height") as u32;
    
  
    HandleResult {
      image: image.resize_exact(w, h, FilterType::Lanczos3),
      format
    }
  }
}

pub struct Crop {}

impl Feat for Crop {
  
  // 裁切
  fn handle(&self, mut image: DynamicImage, param: &JsValue, format: ImageFormat) -> HandleResult {
      
    let w = param_field_number(param, "width") as u32;
    let h = param_field_number(param, "height") as u32;

    let x = param_field_number(param, "x") as u32;
    let y = param_field_number(param, "y") as u32;
    
    HandleResult {
      image: image.crop(x, y, w, h),
      format
    }
  }
}