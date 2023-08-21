mod utils;

use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, ImageFormat};
use std::io::{Cursor, Read, Seek, SeekFrom};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: String);

    #[wasm_bindgen(js_namespace = console, js_name = info)]
    fn info_2(message: String);
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
    info_2(format!("Hello, {}!", name));
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


enum FeatType {
    Grayscale(Grayscale),
    Format(Format)
}


struct Grayscale {}
struct Format {}
trait Feat {}

impl Feat for Grayscale {}
impl Feat for Format {}

#[wasm_bindgen]
pub struct Pipe {
    feats: Vec<Box<dyn Feat>>,
}

#[wasm_bindgen]
impl Pipe {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Pipe {
        Pipe {
            feats: vec![] 
        }
    }

    pub fn add_feature(&mut self, t: &str, value: &str) {
        let feat = Grayscale {};
        self.feats.push(Box::new(feat));
    }

    pub fn feature_len(&self) -> usize {
        self.feats.len()
    }

    pub fn render(arr: &[u8]) {}
}
