
mod utils;
mod feature;

use image::ImageFormat;
use wasm_bindgen::prelude::*;
pub use crate::utils::*;
pub use crate::feature::{Feat, FeatStrategy};



#[wasm_bindgen]
pub struct Pipe {
    feats: Vec<Box<dyn Feat>>,
    params: Vec<JsValue>
}

#[wasm_bindgen]
impl Pipe {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Pipe {
        Pipe {
            feats: vec![],
            params: vec![],
        }
    }

    pub fn add_feature(&mut self, t: &str, value: JsValue) {
        let feat = FeatStrategy::generate(t);
        self.feats.push(feat);
        self.params.push(value);
    }

    pub fn del_feature(&mut self, index: usize) {
        if index >= self.feats.len() {
            return
        }

        self.feats.remove(index);
        self.params.remove(index);
    }


    pub fn set_feature(&mut self, index: usize, param: JsValue) {
        if index >= self.feats.len() {
            return
        }
        self.params[index] = param
    }

    pub fn feature_len(&self) -> usize {
        self.feats.len()
    }

    pub fn render(&self, arr: &[u8], from_suffix: &str) -> String {

        let mut origin_format = ImageFormat::from_extension(from_suffix).expect("no matched extensions");

        let mut dyn_image = buffer_to_image(
            arr, 
            origin_format
        );

        for (i, handler) in self.feats.iter().enumerate() {
            let param = self.params.get(i).expect("params is error");
            let result = handler.handle(dyn_image, param, origin_format);
            dyn_image = result.image;
            origin_format = result.format;
        };

        // HandleResult


        image_to_base64(dyn_image, origin_format)
    }
}
