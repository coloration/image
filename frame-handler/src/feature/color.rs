use image::{DynamicImage, ImageFormat, Rgba, GenericImage};
use wasm_bindgen::prelude::*;
use super::{feat::Feat, HandleResult};
use crate::utils::*;

pub struct Grayscale {}

impl Feat for Grayscale {
    fn handle (&self, image: DynamicImage, _param: &JsValue, format: ImageFormat) -> HandleResult {
        
        HandleResult {
            image: image.grayscale(),
            format
        }
    }
}


pub struct Binary {}

impl Feat for Binary {
    fn handle (&self, mut image: DynamicImage, param: &JsValue, format: ImageFormat) -> HandleResult {

        let t = param_field_number(param, "threshold") as i32;

        let threshold = if t > 255 { 255 } else if t < 0 { 0 } else { t as u8 };
        let gray_image = image.to_luma8();
        let w = image.width();
        let h = image.height();

        for x in 0..w {
            for y in 0..h {
                let gray_p = gray_image.get_pixel(x, y);
                let color_p = Rgba(
                    if gray_p[0] > threshold {[255, 255, 255, 255]}
                    else {[0, 0, 0, 255]}
                );

                image.put_pixel(x, y, color_p);
            }
        }

        HandleResult { image, format }
    }
}


pub struct Invert {}

impl Feat for Invert {
    fn handle (&self, mut image: DynamicImage, _param: &JsValue, format: ImageFormat) -> HandleResult {
        image.invert();
        
        HandleResult { image, format }
    } 
}


pub struct ColorReplace {}

impl Feat for ColorReplace {
    fn handle (&self, mut image: DynamicImage, param: &JsValue, format: ImageFormat) -> HandleResult {
        
        let from = param_field_string(param, "from");
        let to = param_field_string(param, "to");

        let from_rgb = hex_to_rgb(&from);
        let to_rgb = hex_to_rgb(&to);

        let origin = image.to_rgba8();

        let w = image.width();
        let h = image.height();

        for x in 0..w {
            for y in 0..h {
                if origin.get_pixel(x, y) == &from_rgb {
                    image.put_pixel(x, y, to_rgb);
                }
            }
        }

        HandleResult { image, format }
    }
}
