use super::feat::Feat;
use super::format::Format;
use super::color::{ColorReplace, Grayscale, Invert, Binary};
pub struct FeatStrategy {}



impl FeatStrategy {
    pub fn generate(t: &str) -> Box<dyn Feat> {
        match t {
            "grayscale" => Box::new(Grayscale {}),
            "format" => Box::new(Format {}),
            "invert" => Box::new(Invert {}),
            "binary" => Box::new(Binary {}),
            "color-replace" => Box::new(ColorReplace {}),
            _ => {
                panic!("feature type can not matched")
            }
        }
    }
}