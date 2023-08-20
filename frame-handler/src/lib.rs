mod utils;

use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageFormat};
use std::io::{SeekFrom, Cursor, Seek, Read};
use base64::{engine::general_purpose, Engine as _};

fn buffer_to_image(buf: &[u8], format: ImageFormat) -> DynamicImage {
    match image::load_from_memory_with_format(buf, format) {
       Ok(img) => {
        println!("convert succeed!");
        img
       },
       Err(error) => {
        panic!("can not convert the picture: {:?}", error)
       } 
    }
}

fn image_to_base64 (img: DynamicImage, format: ImageFormat) -> String {
    println!("!!!!! hi");
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
    let mut out =  Vec::new();

    c.read_to_end(&mut out).unwrap();

    let stt = general_purpose::STANDARD.encode(&mut out);

    format!("data:{};base64,{}", format.to_mime_type(), stt)
}


// fn load_image_from_array(_array: &[u8]) -> DynamicImage {
//     let img = match image::load_from_memory_with_format(_array, ImageFormat::Png) {
//         Ok(img) => img,
//         Err(error) => {
//             panic!("{:?}", error)
//         }
//     };
    
//     img
// }

// fn get_image_as_base64(_img: DynamicImage) -> String {
//     // 创建一个内存空间
//     let mut c = Cursor::new(Vec::new());
//     match _img.write_to(&mut c, ImageFormat::Png) {
//         Ok(c) => c,
//         Err(error) => {
//             panic!(
//                 "There was a problem writing the resulting buffer: {:?}",
//                 error
//             )
//         }
//     };
//     c.seek(SeekFrom::Start(0)).unwrap();
//     let mut out = Vec::new();
//     // 从内存读取数据
//     c.read_to_end(&mut out).unwrap();
//     // 解码
//     let stt = general_purpose::STANDARD.encode(&mut out);
    
//     format!("{}{}", "data:image/png;base64,", stt)
// }


#[wasm_bindgen]
pub fn grayscale(_array: &[u8]) -> String {

    let mut i = 0;
    for item in _array {
        i += 1;
        println!("{}", item);
        if i == 10 {
            break;
        }
    }

    let origin_format = match ImageFormat::from_extension("png") {
        Some(format) => {
            println!("origin format {:?}", format);
            format
        },
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
