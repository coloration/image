use image::{
  DynamicImage,
  ImageFormat,
  load_from_memory_with_format
}

struct Origin {
  buffer: [u8],
  format: ImageFormat,
  
}


impl Origin {

  new () {

  }

  to_dyn_image(&self) -> DynamicImage {
    match load_from_memory_with_format(&self.buffer, ImageFormat::Png) {
      Ok(image) => image,
      Err(err) => panic!("{:?}", err)
    }
  }

  to_base64(&self, images: &Vec<DynamicImage>) -> Vec<String> {
    vec![]
  }
}

struct Feature {

}

struct Pipe {
  origins: Vec<Origin>
  handlers: Vec<Feature>
  results: Vec<&str>
}


impl Pipe {
  
  new (buffers: &[[u8]], formats: &[ImageFormat], handlers: &[Feature]) -> Pipe {
    
    for (i, bf) in buffers.iter() {
      match Origin::new(bf, &formats[i]) {
        Ok(origin) => self.origins.push(origin),
        Err(e) => {},
      }
    }
  }

  run (&mut self) {
    self.result = vec![];
    for origin in self.origins {
      let mut img = origin.to_dyn_images();

      for handler in &self.handlers {
        img = handler.exce(img).nowrap();
      }

      
      self.result.push(Feature.to_base64(img))
    }
  }
  
}