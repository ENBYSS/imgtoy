use std::path::Path;

use ez_ffmpeg::{stream_info::StreamInfo, FfmpegContext, FfmpegScheduler, Input, Output};
use image::DynamicImage;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::utils::resource::image::parser::ImageParser;

pub struct FfmpegUtil {}

impl FfmpegUtil {
    pub fn demo() {
        let source = "https://images-ext-1.discordapp.net/external/7yNX5iM1y5yWlDVWngAIYqqozZxdci-gxuScOTZk028/https/media.tenor.com/KB7xapJWhPMAAAPo/ggg-great-god-grove.mp4";
        // Self::get_fps_of_file(source);
        // Self::split_media_into_frames(source, "demo");
        // Self::combine_frames_into_file("demo", "output/ffmpeg-demo.mp4", 0.0, true);
        panic!("END OF FFMPEG DEMO");
    }

    pub fn split_media(input: &str, ffmpeg_path: FfmpegPathUtil) -> (Vec<DynamicImage>, f64) {
        let VideoInfo { fps, frame_count } = Self::get_video_info(input);
        Self::split_media_into_frames(input, ffmpeg_path.clone());

        // let mut frames = Vec::with_capacity(frame_count as usize);

        (
            (1..frame_count + 1)
                .into_par_iter()
                .map(|i| {
                    let image_name = ffmpeg_path.frame_path_rs(i);
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
        path_util: FfmpegPathUtil,
        out: &str,
    ) {
        let dir = path_util.dir();
        if !Path::new(&dir).is_dir() {
            std::fs::create_dir_all(&dir).unwrap();
        }

        frames.iter().enumerate().for_each(|(i, frame)| {
            let i = i + 1;
            frame.save(path_util.frame_path_rs(i as i64)).unwrap()
        });

        Self::combine_frames_into_file(path_util, out, frame_rate, true);
    }

    pub fn clear_temp(temp_prefix: &str) {
        std::fs::remove_dir_all(format!("temp/{temp_prefix}")).unwrap();
    }

    // accepts gif, images, etc...
    pub fn split_media_into_frames(input: &str, temp_prefix: FfmpegPathUtil) {
        // ffmpeg -i input/Clap.gif -vsync 0 temp/temp%d.png
        let temp_dir = temp_prefix.dir();
        if !Path::new(&temp_dir).is_dir() {
            std::fs::create_dir_all(&temp_dir).unwrap();
        }

        let context =
            FfmpegContext::builder()
                .input(Input::from(input).set_hwaccel("cuda"))
                .output(Output::from(temp_prefix.frame_path()).set_vsync_method(
                    ez_ffmpeg::core::context::output::VSyncMethod::VsyncPassthrough,
                ))
                // .output(Output::from(temp_prefix.audio_path()))
                .build()
                .unwrap();

        FfmpegScheduler::new(context)
            .start()
            .unwrap()
            .wait()
            .unwrap();

        let context = FfmpegContext::builder()
            .input(Input::from(input).set_hwaccel("cuda"))
            // .output(Output::from(temp_prefix.frame_path()).set_vsync_method(
            //     ez_ffmpeg::core::context::output::VSyncMethod::VsyncPassthrough,
            // ))
            .output(Output::from(temp_prefix.audio_path()))
            .build()
            .unwrap();

        FfmpegScheduler::new(context)
            .start()
            .unwrap()
            .wait()
            .unwrap();
    }

    pub fn combine_frames_into_file(
        ffmpeg_path: FfmpegPathUtil,
        out: &str,
        frame_rate: f64,
        audio: bool,
    ) {
        // ffmpeg -f image2 -framerate 9 -i image_%003d.jpg -vf scale=531x299,transpose=1,crop=299,431,0,100 out.gif
        let mut context = FfmpegContext::builder()
            .input(Input::from(ffmpeg_path.frame_path()).set_format("image2"))
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
        let audio_path = ffmpeg_path.audio_path();
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

#[derive(Clone)]
pub struct FfmpegPathUtil {
    pub temp: bool,
    pub prefix: String,
}

impl FfmpegPathUtil {
    fn _dir(&self) -> String {
        if self.temp {
            "temp".into()
        } else {
            ".".into()
        }
    }

    pub fn dir(&self) -> String {
        let dir = self._dir();
        format!("{}/{}", dir, self.prefix)
    }

    pub fn frame_path(&self) -> String {
        format!("{}/frame-%04d.png", self.dir())
    }

    pub fn frame_path_rs(&self, i: i64) -> String {
        format!("{}/frame-{i:04}.png", self.dir())
    }

    pub fn audio_path(&self) -> String {
        format!("{}/source/audio.mp3", self._dir())
    }
}

struct VideoInfo {
    fps: f64,
    frame_count: i64,
}
