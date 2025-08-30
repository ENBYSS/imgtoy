use image::{DynamicImage, GenericImageView};
use image_effects::dispatch::EffectEnum;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    parsers::v2::structure::meta::{SizeConstraint, SourceKind},
    utils::resource::ffmpeg::utils::{FfmpegPathUtil, FfmpegUtil},
    utils::resource::image::parser::ImageKind,
};

#[derive(Clone)]
pub struct FfmpegProcessor {
    source: String,
    image_kind: ImageKind,
    frames: Vec<DynamicImage>,
    fps: f64,
    prefix: String,
}

impl FfmpegProcessor {
    pub fn use_source(source: &SourceKind) -> Self {
        let image_kind = source.get_image_kind();
        let source_path = source.get_path();
        let prefix = "source";

        let (frames, fps) = match image_kind {
            ImageKind::Image => todo!(),
            ImageKind::Gif | ImageKind::Anim => {
                Self::process_animated(&source_path, Self::_gen_path_util(prefix))
            }
        };

        println!("DETECTED {fps}fps");

        Self {
            source: source_path,
            image_kind,
            frames,
            fps,
            prefix: prefix.into(),
        }
    }

    pub fn gen_path_util(&self) -> FfmpegPathUtil {
        Self::_gen_path_util(&self.prefix)
    }

    fn _gen_path_util(prefix: &str) -> FfmpegPathUtil {
        FfmpegPathUtil {
            temp: true,
            prefix: (prefix).into(),
        }
    }

    pub fn constrain(mut self, constrain: &SizeConstraint) -> Self {
        self.frames = self
            .frames
            .into_iter()
            .map(|frame| constrain.constrain(frame))
            .collect();
        self
    }

    pub fn set_prefix(&mut self, prefix: impl ToString) {
        self.prefix = prefix.to_string();
    }

    pub fn save(&self, out: &str) {
        let extension = match self.image_kind {
            ImageKind::Gif => "gif",
            ImageKind::Anim => "mp4",
            ImageKind::Image => panic!("yeah something went v. weird here"),
        };

        FfmpegUtil::combine_media(
            &self.frames,
            self.fps,
            self.gen_path_util(),
            format!("{out}.{extension}").as_str(),
        );
    }

    fn process_animated(path: &str, prefix: FfmpegPathUtil) -> (Vec<DynamicImage>, f64) {
        FfmpegUtil::split_media(path, prefix)
    }

    pub fn apply_effects(mut self, effects: Vec<EffectEnum<DynamicImage>>) -> Self {
        for effect in effects.into_iter() {
            self.frames = self
                .frames
                .into_par_iter()
                .map(|frame| effect.affect(frame))
                .collect();
        }

        self
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        println!("WE HAVE {} frames", self.frames.len());
        self.frames.get(0).unwrap().dimensions()
    }

    pub fn clear_temp(&self) {
        FfmpegUtil::clear_temp(&self.prefix);
    }
}
