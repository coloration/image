mod utils;

use wasm_bindgen::prelude::*;
use image::{DynamicImage,ImageFormat};
use std::io::*;
use base64::{engine::general_purpose, Engine as _};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, frame-handler!");
}

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1
    }
    fib(n - 1) + fib(n - 2)
}


fn load_image_from_array(_array: &[u8]) -> DynamicImage {
    let img = match image::load_from_memory_with_format(_array, ImageFormat::Png) {
        Ok(img) => img,
        Err(error) => {
            panic!("{:?}", error)
        }
    };
    
    img
}

fn get_image_as_base64(_img: DynamicImage) -> String {
    // 创建一个内存空间
    let mut c = Cursor::new(Vec::new());
    match _img.write_to(&mut c, ImageFormat::Png) {
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
    // 从内存读取数据
    c.read_to_end(&mut out).unwrap();
    // 解码
    let stt = general_purpose::STANDARD.encode(&mut out);
    
    format!("{}{}", "data:image/png;base64,", stt)
}


#[wasm_bindgen]
pub fn grayscale(_array: &[u8]) -> String {
    let mut img = load_image_from_array(_array);
    img = img.grayscale();
    
    get_image_as_base64(img)
}
