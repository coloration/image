use super::feat::Feat;
use super::grayscale::Grayscale;
use super::invert::Invert;
use super::format::Format;
use super::binary::Binary;

pub struct FeatStrategy {}



impl FeatStrategy {
    pub fn generate(t: &str) -> Box<dyn Feat> {
        match t {
            "grayscale" => Box::new(Grayscale {}),
            "format" => Box::new(Format {}),
            "invert" => Box::new(Invert {}),
            "binary" => Box::new(Binary {}),
            _ => {
                panic!("feature type can not matched")
            }
        }
    }
}