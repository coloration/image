use image::{DynamicImage, ImageFormat};
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use crate::log;

use super::{feat::Feat, HandleResult};

pub struct Format {}

impl Feat for Format {
    fn handle (&self, image: DynamicImage, param: &JsValue, _format: ImageFormat) -> HandleResult {
        let suffix = Reflect::get(param, &"format".into()).expect("error param [format] field.")
            .as_string().expect("can not convert [format] field");

        log(format!("format: {}", suffix));


        let format = ImageFormat::from_extension(suffix)
        .expect("can not convert [format] field");
        HandleResult {
            image,
            format
        }
    }
}