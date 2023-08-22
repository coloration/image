mod utils;

use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, ImageFormat};
use std::io::{Cursor, Read, Seek, SeekFrom};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: String);
}

fn buffer_to_image(buf: &[u8], format: ImageFormat) -> DynamicImage {
    match image::load_from_memory_with_format(buf, format) {
        Ok(img) => {
            println!("convert succeed!");
            img
        }
        Err(error) => {
            panic!("can not convert the picture: {:?}", error)
        }
    }
}

fn image_to_base64(img: DynamicImage, format: ImageFormat) -> String {
    let mut c = Cursor::new(Vec::new());
    match img.write_to(&mut c, format) {
        Ok(c) => c,
        Err(error) => {
            panic!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            )
        }
    };

    c.seek(SeekFrom::Start(0)).unwrap();
    let mut out = Vec::new();

    c.read_to_end(&mut out).unwrap();

    let stt = general_purpose::STANDARD.encode(&mut out);

    format!("data:{};base64,{}", format.to_mime_type(), stt)
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(format!("Hello, {}!", name));
}

// example
#[wasm_bindgen]
pub fn grayscale(_array: &[u8]) -> String {
    let mut i = 0;
    for item in _array {
        i += 1;
        log(format!("{}", item));
        if i == 10 {
            break;
        }
    }

    let origin_format = match ImageFormat::from_extension("png") {
        Some(format) => {
            println!("origin format {:?}", format);
            format
        }
        None => {
            panic!("error suffix!")
        }
    };

    let target_format = match ImageFormat::from_extension("jpg") {
        Some(format) => format,
        None => {
            panic!("error suffix!")
        }
    };

    let mut img = buffer_to_image(_array, origin_format);
    img = img.grayscale();

    image_to_base64(img, target_format)
}



trait Feat {
    
    fn handle(&self, img: DynamicImage, param: &JsValue) -> DynamicImage;
    
}


struct Grayscale {
}

impl Feat for Grayscale {
    fn handle (&self, img: DynamicImage, _param: &JsValue) -> DynamicImage {
        img.grayscale()
    }
}


struct Invert {
}

impl Feat for Invert {
    fn handle (&self, mut img: DynamicImage, _param: &JsValue) -> DynamicImage {
        img.invert();
        img
    } 
}

struct Format {
}

impl Feat for Format {
    fn handle (&self, img: DynamicImage, _param: &JsValue) -> DynamicImage {
        img
    }
}


struct FeatStrategy {}

impl FeatStrategy {
    fn generate(t: &str) -> Box<dyn Feat> {
        match t {
            "grayscale" => Box::new(Grayscale {}),
            "format" => Box::new(Format {}),
            "invert" => Box::new(Invert {}),
            _ => {
                panic!("feature type can not matched")
            }
        }
    }
}

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

    pub fn render(&self, arr: &[u8], from_suffix: &str, to_suffix:&str) -> String {
        let mut dyn_image = buffer_to_image(
            arr, 
            ImageFormat::from_extension(from_suffix).expect("no matched extensions")
        );

        for (i, handler) in self.feats.iter().enumerate() {
            let param = self.params.get(i).expect("params is error");
            dyn_image = handler.handle(dyn_image, param);
        };


        image_to_base64(
            dyn_image,
            ImageFormat::from_extension(to_suffix).expect("no matched extensions")
        )
    }
}
