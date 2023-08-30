use super::feat::Feat;
use super::format::Format;
use super::color::{ColorReplace, Grayscale, Invert, Binary};
use super::shape::{Resize, Crop};
pub struct FeatStrategy {}



impl FeatStrategy {
    pub fn generate(t: &str) -> Box<dyn Feat> {
        match t {
            "grayscale" => Box::new(Grayscale {}),
            "invert" => Box::new(Invert {}),
            "binary" => Box::new(Binary {}),
            "color-replace" => Box::new(ColorReplace {}),
            
            "format" => Box::new(Format {}),

            "resize" => Box::new(Resize {}),
            "crop" => Box::new(Crop {}), 

            _ => {
                panic!("feature type can not matched")
            }
        }
    }
}