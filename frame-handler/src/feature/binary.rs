use image::{DynamicImage, GenericImage, Rgba, ImageFormat};
use wasm_bindgen::prelude::*;
use super::{feat::Feat, HandleResult};
use js_sys::Reflect;
use crate::log;

pub struct Binary {}

impl Feat for Binary {
    fn handle (&self, mut image: DynamicImage, param: &JsValue, format: ImageFormat) -> HandleResult {
        

        let t = Reflect::get(param, &"threshold".into()).expect("error param [threshold] field.")
            .as_f64().expect("can not convert [threshold] field to number")
            .floor() as i64;
        log(format!("threshold: {}", t));

        let threshold = if t > 255 { 255 } else if t < 0 { 0 } else { t as u8 };
        let gray_image = image.to_luma8();
        let w = image.width();
        let h = image.height();

        for x in 0..w {
            for y in 0..h {
                let gray_p = gray_image.get_pixel(x, y);

                image.put_pixel(
                    x, 
                    y, 
                    Rgba(if gray_p[0] > threshold {[255, 255, 255, 255]} else {[0, 0, 0, 255]}));
                // if (gray_p[0] > threshold) 255
            }
        }

        HandleResult { image, format }
    }
}
