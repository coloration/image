use base64::{engine::general_purpose, Engine as _};
use std::io::{Cursor, Read, Seek, SeekFrom};
use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageFormat};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(message: String);
}

pub fn buffer_to_image(buf: &[u8], format: ImageFormat) -> DynamicImage {
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

pub fn image_to_base64(img: DynamicImage, format: ImageFormat) -> String {
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
