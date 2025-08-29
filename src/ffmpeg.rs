use ez_ffmpeg::{stream_info::StreamInfo, FfmpegContext, FfmpegScheduler, Input, Output};
use image::DynamicImage;
use image_effects::dispatch::EffectEnum;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    parsers::v2::structure::meta::{Source, SourceKind},
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
    pub fn use_source(source: SourceKind, prefix: impl ToString) -> Self {
        let image_kind = source.get_image_kind();
        let prefix = prefix.to_string();

        match image_kind {
            ImageKind::Image => Self::ImageResource(ImageResource::use_source(&source)),
            ImageKind::Gif | ImageKind::Anim => {
                Self::FfmpegProcessor(FfmpegProcessor::use_source(&source, &prefix))
            }
        }
    }

    pub fn apply_effects(mut self, effects: Vec<EffectEnum<DynamicImage>>) -> Self {
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

    pub fn apply_effects(mut self, effects: Vec<EffectEnum<DynamicImage>>) -> Self {
        for effect in effects.into_iter() {
            self.image = effect.affect(self.image);
        }

        self
    }

    pub fn save(&self, out: &str) {
        self.image.save(out).unwrap();
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
            ImageKind::Gif | ImageKind::Anim => Self::process_animated(&source_path, &prefix),
        };

        Self {
            source: source_path,
            image_kind,
            frames,
            fps,
            prefix,
        }
    }

    pub fn save(&self, out: &str) {
        FfmpegUtil::combine_media(&self.frames, self.fps, &self.prefix, out);
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
        Self::combine_frames_into_file("demo", "output/ffmpeg-demo.mp4", 0.0);
        panic!("END OF FFMPEG DEMO");
    }

    pub fn split_media(input: &str, temp_prefix: &str) -> (Vec<DynamicImage>, f64) {
        let VideoInfo { fps, frame_count } = Self::get_video_info(input);
        Self::split_media_into_frames(input, temp_prefix);

        // let mut frames = Vec::with_capacity(frame_count as usize);

        (
            (0..frame_count)
                .into_par_iter()
                .map(|i| {
                    let image_name = format!("temp/{temp_prefix}-frame-{i:04}.png");
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
        frames.iter().enumerate().for_each(|(i, frame)| {
            frame
                .save(format!("temp/output-{temp_prefix}-frame-{i:04}.png"))
                .unwrap()
        });

        Self::combine_frames_into_file(temp_prefix, out, frame_rate);
    }

    // accepts gif, images, etc...
    pub fn split_media_into_frames(input: &str, temp_prefix: &str) {
        // ffmpeg -i input/Clap.gif -vsync 0 temp/temp%d.png
        let context = FfmpegContext::builder()
            .input(Input::from(input))
            .output(
                Output::from(format!("temp/{temp_prefix}-frame-%04d.png")).set_vsync_method(
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
    }

    pub fn combine_frames_into_file(temp_prefix: &str, out: &str, frame_rate: f64) {
        // ffmpeg -f image2 -framerate 9 -i image_%003d.jpg -vf scale=531x299,transpose=1,crop=299,431,0,100 out.gif
        let context = FfmpegContext::builder()
            .input(Input::from(format!("temp/{temp_prefix}-frame-%d.png")).set_format("image2"))
            // .filter_descs(vec![
            //     "scale=X.Y",        // change scale of output
            //     "transpose=1",      // rotate 90 degrees clockwise
            //     "crop=0,0,0,0",     // crop something
            // ])
            .output(Output::from(out).set_framerate(ez_ffmpeg::AVRational {
                num: frame_rate as i32,
                den: 0,
            }))
            .build()
            .unwrap();

        FfmpegScheduler::new(context)
            .start()
            .unwrap()
            .wait()
            .unwrap();
    }

    pub fn get_video_info(input: &str) -> VideoInfo {
        // FfmpegContext::builder().input(input)
        let meta = ez_ffmpeg::stream_info::find_video_stream_info(input).unwrap();
        println!("{meta:#?}");
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
