use image::DynamicImage;

struct FileSuffix(String);


pub enum FeatureFormat {
  Png,
  Jpg,
  Ico,
  // ...
}

pub fn static_format(image: DynamicImage, suffix: FeatureFormat) -> DynamicImage {
  image
}