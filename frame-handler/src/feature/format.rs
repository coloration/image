use image::{DynamicImage, ImageFormat};
use wasm_bindgen::prelude::*;
use crate::utils::*;

use super::{feat::Feat, HandleResult};

pub struct Format {}

impl Feat for Format {
    fn handle (&self, image: DynamicImage, param: &JsValue, _format: ImageFormat) -> HandleResult {
        
        let suffix = param_field_string(param, "format");

        let format = ImageFormat::from_extension(suffix)
        .expect("can not convert [format] field");
        HandleResult {
            image,
            format
        }
    }
}