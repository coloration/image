use image::{DynamicImage};

// 灰度化
pub fn grayscale(image: DynamicImage) -> DynamicImage {
  img.grayscale()
}

// 二值化
pub fn binary(iamge: DynamicImage, boundary: i8) -> DynamicImage {
  img.binary()
}

// 替换目标颜色，透明底色
pub fn replace(
  image: DynamicImage, 
  target: &[u8], 
  to: &[u8],
  range: u8
) -> DynamicImage {
  image
}

// 反相, 反色
pub fn reverse(image: DynamicImage) -> DynamicImage {
  image
}