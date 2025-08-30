use std::{os::unix::process, path::Path};

use ez_ffmpeg::{
    error::AllocOutputContextError, stream_info::StreamInfo, FfmpegContext, FfmpegScheduler, Input,
    Output,
};
use image::{DynamicImage, GenericImageView};
use image_effects::dispatch::EffectEnum;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    parsers::v2::structure::meta::{SizeConstraint, Source, SourceKind},
    utils::image::{ImageKind, ImageParser},
};

/// # Ideas
/// ## OpenGL
/// Look into using OpenGL for effects? Maybe generate shader code?
/// ## Indepth
/// Look deeper into ffmpeg commands?

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
                Self::FfmpegProcessor(FfmpegProcessor::use_source(source, String::new()))
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

#[derive(Clone)]
pub struct FfmpegProcessor {
    source: String,
    image_kind: ImageKind,
    frames: Vec<DynamicImage>,
    fps: f64,
    prefix: String,
}

impl FfmpegProcessor {
    pub fn use_source(source: &SourceKind, prefix: impl ToString) -> Self {
        let image_kind = source.get_image_kind();
        let source_path = source.get_path();
        let prefix = prefix.to_string();

        let (frames, fps) = match image_kind {
            ImageKind::Image => todo!(),
            ImageKind::Gif | ImageKind::Anim => Self::process_animated(&source_path, "source"),
        };

        println!("DETECTED {fps}fps");

        Self {
            source: source_path,
            image_kind,
            frames,
            fps,
            prefix,
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
            &self.prefix,
            format!("{out}.{extension}").as_str(),
        );
    }

    fn process_animated(path: &str, prefix: &str) -> (Vec<DynamicImage>, f64) {
        FfmpegUtil::split_media(path, &prefix)
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

struct VideoInfo {
    fps: f64,
    frame_count: i64,
}

pub struct FfmpegUtil {}

impl FfmpegUtil {
    pub fn demo() {
        let source = "https://images-ext-1.discordapp.net/external/7yNX5iM1y5yWlDVWngAIYqqozZxdci-gxuScOTZk028/https/media.tenor.com/KB7xapJWhPMAAAPo/ggg-great-god-grove.mp4";
        // Self::get_fps_of_file(source);
        Self::split_media_into_frames(source, "demo");
        Self::combine_frames_into_file("demo", "output/ffmpeg-demo.mp4", 0.0, true);
        panic!("END OF FFMPEG DEMO");
    }

    pub fn split_media(input: &str, temp_prefix: &str) -> (Vec<DynamicImage>, f64) {
        let VideoInfo { fps, frame_count } = Self::get_video_info(input);
        Self::split_media_into_frames(input, temp_prefix);

        // let mut frames = Vec::with_capacity(frame_count as usize);

        (
            (1..frame_count + 1)
                .into_par_iter()
                .map(|i| {
                    let image_name = format!("temp/{temp_prefix}/frame-{i:04}.png");
                    let frame = ImageParser::parse_localfile(&image_name).into_image();
                    // frames.push(frame);
                    frame
                })
                .collect(),
            fps,
        )
    }

    pub fn combine_media(
        frames: &Vec<DynamicImage>,
        frame_rate: f64,
        temp_prefix: &str,
        out: &str,
    ) {
        let temp_dir = format!("temp/{temp_prefix}");
        if !Path::new(&temp_dir).is_dir() {
            std::fs::create_dir_all(&temp_dir).unwrap();
        }

        frames.iter().enumerate().for_each(|(i, frame)| {
            let i = i + 1;
            frame
                .save(format!("temp/{temp_prefix}/frame-{i:04}.png"))
                .unwrap()
        });

        Self::combine_frames_into_file(temp_prefix, out, frame_rate, true);
    }

    pub fn clear_temp(temp_prefix: &str) {
        std::fs::remove_dir_all(format!("temp/{temp_prefix}")).unwrap();
    }

    // accepts gif, images, etc...
    pub fn split_media_into_frames(input: &str, temp_prefix: &str) {
        // ffmpeg -i input/Clap.gif -vsync 0 temp/temp%d.png
        let temp_dir = format!("temp/{temp_prefix}");
        if !Path::new(&temp_dir).is_dir() {
            std::fs::create_dir_all(&temp_dir).unwrap();
        }

        let context = FfmpegContext::builder()
            .input(Input::from(input).set_hwaccel("cuda"))
            .output(
                Output::from(format!("temp/{temp_prefix}/frame-%04d.png")).set_vsync_method(
                    ez_ffmpeg::core::context::output::VSyncMethod::VsyncPassthrough,
                ),
            )
            .build()
            .unwrap();

        FfmpegScheduler::new(context)
            .start()
            .unwrap()
            .wait()
            .unwrap();

        let audio_path = format!("temp/{temp_prefix}/audio.mp3");
        println!("TRIED TO PARSE: {audio_path}");

        let context = FfmpegContext::builder()
            .input(Input::from(input))
            .output(Output::from(audio_path))
            .build()
            .unwrap();

        FfmpegScheduler::new(context)
            .start()
            .unwrap()
            .wait()
            .unwrap();
    }

    pub fn combine_frames_into_file(temp_prefix: &str, out: &str, frame_rate: f64, audio: bool) {
        let temp_dir = format!("temp/{temp_prefix}");
        let input = format!("{temp_dir}/frame-%04d.png");

        // ffmpeg -f image2 -framerate 9 -i image_%003d.jpg -vf scale=531x299,transpose=1,crop=299,431,0,100 out.gif
        let mut context = FfmpegContext::builder()
            .input(Input::from(input).set_format("image2"))
            // .filter_descs(vec![
            //     "scale=X.Y",        // change scale of output
            //     "transpose=1",      // rotate 90 degrees clockwise
            //     "crop=0,0,0,0",     // crop something
            // ])
            .output(
                Output::from(out)
                    .set_framerate(ez_ffmpeg::AVRational {
                        num: frame_rate as i32,
                        den: 1,
                    })
                    .set_audio_codec("mp3"),
            );

        // implement audio support?
        let audio_path = format!("temp/source/audio.mp3");
        // println!("TRIED TO PARSE: {audio_path}");

        if audio {
            context = context.input(Input::from(audio_path));
        }

        let context = context.build().unwrap();

        FfmpegScheduler::new(context)
            .start()
            .unwrap()
            .wait()
            .unwrap();
    }

    pub fn get_video_info(input: &str) -> VideoInfo {
        let meta = ez_ffmpeg::stream_info::find_video_stream_info(input).unwrap();
        let stream_info = meta.unwrap();
        let (fps, nb_frames) = match stream_info {
            StreamInfo::Video { fps, nb_frames, .. } => (fps, nb_frames),
            _ => unimplemented!("???"),
        };

        VideoInfo {
            fps,
            frame_count: nb_frames,
        }
    }

    // dummy function to list all properties of a stream_info video
    fn get_video_stream_info(input: &str) {
        let stream_info = ez_ffmpeg::stream_info::find_video_stream_info(input)
            .unwrap()
            .unwrap();

        match stream_info {
            StreamInfo::Video {
                index,
                time_base,
                start_time,
                duration,
                nb_frames,
                r_frame_rate,
                sample_aspect_ratio,
                metadata,
                avg_frame_rate,
                codec_id,
                codec_name,
                width,
                height,
                bit_rate,
                pixel_format,
                video_delay,
                fps,
                rotate,
            } => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}
