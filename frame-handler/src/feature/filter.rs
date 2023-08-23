use image::DynamicImage;



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