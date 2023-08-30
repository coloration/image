use base64::{engine::general_purpose, Engine as _};
use std::io::{Cursor, Read, Seek, SeekFrom};
use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageFormat, Rgba};
use js_sys::Reflect;


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


pub fn get_param_field(param: &JsValue, field: &str) -> JsValue {
  match Reflect::get(param, &JsValue::from(field)) {
    Ok(v) => v,
    Err(err) => {
      log(format!("error param [{}] field.", field));
      panic!("{:?}", err);
    }
  }
}

pub fn param_field_number(param: &JsValue, field: &str) -> f64 {
  match get_param_field(param, field).as_f64() {
    Some(v) => v,
    None => {
      log(format!("can not convert [{}] field to number", field));
      panic!()
    }
  }
}

pub fn param_field_string(param: &JsValue, field: &str) -> String {
  match get_param_field(param, field).as_string() {
    Some(v) => v,
    None => {
      log(format!("can not convert [{}] field to string", field));
      panic!()
    }
  }
}

pub fn param_field_boolean(param: &JsValue, field: &str) -> bool {
  match get_param_field(param, field).as_bool() {
    Some(v) => v,
    None => {
      log(format!("can not convert [{}] field to boolean", field));
      panic!()
    }
  }
}


const ALL_CHARS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_";

/// 进阶版 10 进制转 11 - 64 进制
pub fn base_10_to_n(num: u64, radix: u32) -> String {
    if num == 0 {
        return String::from("0");
    }

    let base = base_10_to_n(num / (radix as u64), radix);
    let start = base.strip_prefix("0").unwrap_or(base.as_str());
    let end = match ALL_CHARS.chars().nth((num % (radix as u64)) as usize) {
        Some(data) => String::from(data),
        _ => String::from(""),
    };
    format!("{}{}", start, end)
}

/// 11 - 64 进制解析为 10 进制
///
/// ```
/// let id = "1gbyra5idyk8r";
/// let raw_id = 6888076346770202619;
/// assert_eq!(base_n_to_10(id, 36), 6888076346770202619);
/// ```
pub fn base_n_to_10(num_str: &str, radix: u32) -> u32 {
    let mut result: u32 = 0;
    for i in 0..num_str.len() {
        result *= radix as u32;
        let target_char = num_str.chars().nth(i).unwrap_or('0');
        let data = ALL_CHARS.chars().position(|i| i == target_char).unwrap_or(0);
        result += data as u32;
    }
    result
}

pub fn hex_to_rgb(hex_str: &str) -> Rgba<u8> {
  let hex_r = &hex_str[1..3];
  let hex_g = &hex_str[3..5];
  let hex_b = &hex_str[5..7];
  
  Rgba ([
    base_n_to_10(hex_r, 16) as u8,
    base_n_to_10(hex_g, 16) as u8,
    base_n_to_10(hex_b, 16) as u8,
    255
  ])
}