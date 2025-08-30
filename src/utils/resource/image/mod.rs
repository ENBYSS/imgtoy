pub mod parser;

use image::{DynamicImage, GenericImageView};
use image_effects::dispatch::EffectEnum;

use crate::{
    parsers::v2::structure::meta::{SizeConstraint, SourceKind},
    utils::resource::image::parser::ImageParser,
};

#[derive(Clone)]
pub struct ImageResource {
    image: DynamicImage,
}

impl From<DynamicImage> for ImageResource {
    fn from(value: DynamicImage) -> Self {
        Self { image: value }
    }
}

impl ImageResource {
    pub fn use_source(source: &SourceKind) -> Self {
        Self {
            image: ImageParser::parse_file(&source).into_image(),
        }
    }

    pub fn constrain(mut self, constrain: &SizeConstraint) -> Self {
        self.image = constrain.constrain(self.image);
        self
    }

    pub fn apply_effects(mut self, effects: Vec<EffectEnum<DynamicImage>>) -> Self {
        for effect in effects.into_iter() {
            self.image = effect.affect(self.image);
        }

        self
    }

    pub fn save(&self, out: &str) {
        self.image.save(format!("{}.png", out)).unwrap();
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        self.image.dimensions()
    }
}
