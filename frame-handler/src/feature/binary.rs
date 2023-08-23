use image::{DynamicImage, GenericImage, Rgba};
use wasm_bindgen::prelude::*;
use super::feat::Feat;

pub struct Binary {}

impl Feat for Binary {
    fn handle (&self, mut img: DynamicImage, _param: &JsValue) -> DynamicImage {
        let threshold = 128;
        let gray_img = img.to_luma8();
        let w = img.width();
        let h = img.height();

        for x in 0..w {
            for y in 0..h {
                let gray_p = gray_img.get_pixel(x, y);

                img.put_pixel(
                    x, 
                    y, 
                    Rgba(if gray_p[0] > threshold {[255, 255, 255, 255]} else {[0, 0, 0, 255]}));
                // if (gray_p[0] > threshold) 255
            }
        }

        img 
    }
}
