use image::{DynamicImage, ImageFormat, Rgba, GenericImage};
use wasm_bindgen::prelude::*;
use super::{feat::Feat, HandleResult};
use js_sys::Reflect;
use crate::log;
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
        
        let from = Reflect::get(param, &"from".into())
            .expect("not found: param [from] field")
            .as_string()
            .expect("can not convert [from] field to string");

        
        let to = Reflect::get(param, &"to".into())
        .expect("not found: param [to] field")
        .as_string()
        .expect("can not convert [to] field to string");

        let from_rgb = hex_to_rgb(&from);
        let to_rgb = hex_to_rgb(&to);

        let origin = image.to_rgba8();

        let w = image.width();
        let h = image.height();

        log(format!("from color: {:?}", from_rgb));
        log(format!("to color: {:?}", to_rgb));
        log(format!("first color: {:?}", origin.get_pixel(0, 0)));

        for x in 0..w {
            for y in 0..h {
                let p = origin.get_pixel(x, y);

                if p == &from_rgb {
                    
                    image.put_pixel(x, y, to_rgb);

                    if x == 0 && y == 0 {
                        log(format!("matched color"));
                    }
                }
              
            }
        }
        

        HandleResult {
            image,
            format
        }
    }
}
