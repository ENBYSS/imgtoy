use ::image::DynamicImage;
use image_effects::dispatch::EffectEnum;

use crate::{
    parsers::v2::structure::meta::{SizeConstraint, SourceKind},
    utils::{
        resource::image::parser::ImageKind,
        resource::{ffmpeg::processor::FfmpegProcessor, image::ImageResource},
    },
};

pub mod ffmpeg;
pub mod image;

#[derive(Clone)]
pub enum Resource {
    ImageResource(ImageResource),
    FfmpegProcessor(FfmpegProcessor),
}

impl Resource {
    pub fn use_source(source: &SourceKind) -> Self {
        let image_kind = source.get_image_kind();

        match image_kind {
            ImageKind::Image => Self::ImageResource(ImageResource::use_source(source)),
            ImageKind::Gif | ImageKind::Anim => {
                Self::FfmpegProcessor(FfmpegProcessor::use_source(source))
            }
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        match self {
            Self::FfmpegProcessor(processor) => processor.get_dimensions(),
            Self::ImageResource(resource) => resource.get_dimensions(),
        }
    }

    pub fn with_prefix(&self, prefix: impl ToString) -> Self {
        match self {
            Self::FfmpegProcessor(processor) => {
                let mut processor = processor.clone();
                processor.set_prefix(prefix);
                Self::FfmpegProcessor(processor)
            }
            Self::ImageResource(resource) => Self::ImageResource(resource.clone()),
        }
    }

    pub fn apply_effects(self, effects: Vec<EffectEnum<DynamicImage>>) -> Self {
        match self {
            Self::FfmpegProcessor(processor) => {
                Self::FfmpegProcessor(processor.apply_effects(effects))
            }
            Self::ImageResource(resource) => Self::ImageResource(resource.apply_effects(effects)),
        }
    }

    pub fn save(&self, path: &str) {
        match self {
            Self::FfmpegProcessor(processor) => processor.save(path),
            Self::ImageResource(resource) => resource.save(path),
        }
    }

    pub fn constrain(mut self, constraint: &SizeConstraint) -> Self {
        match self {
            Self::FfmpegProcessor(processor) => {
                Self::FfmpegProcessor(processor.constrain(constraint))
            }
            Self::ImageResource(resource) => Self::ImageResource(resource.constrain(constraint)),
        }
    }

    pub fn clear_temp(&self) {
        match self {
            Self::ImageResource(_) => {}
            Self::FfmpegProcessor(processor) => processor.clear_temp(),
        }
    }
}
