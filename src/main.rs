use std::{error::Error, fs::File, path::Path, time::Duration};

use image::{codecs::gif::GifEncoder, DynamicImage, Frame};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{parsers::v2::structure::MainConfiguration, utils::image::ImageResult};

mod ffmpeg;
// logging is unused, since it works w/ v1.
// a v2 version should be made, making use of enum dispatch.
// mod logging;
mod clap;
mod parsers;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    // crate::clap::clap_demo();
    // crate::ffmpeg::FfmpegUtil::demo();

    let mut args = std::env::args();

    let label_processing = "[...]".blue();
    let label_info = "[ @ ]".purple();
    let label_alert = "[ ! ]".yellow();

    let label_processing = label_processing.bold();
    let label_info = label_info.bold();
    let label_alert = label_alert.bold();

    if args.len() != 2 {
        panic!("Expected a single arg which represents the filepath of the configuration file.");
    }

    let config_file = args.nth(1).unwrap();

    println!("{label_processing} | Reading configuration file: {config_file}");

    let config = std::fs::read_to_string(config_file)?;

    println!("{label_processing} | Parsing file as YAML...");

    let yaml: serde_yaml::Value = serde_yaml::from_str(&config)?;

    println!("{label_processing} | Parsing YAML as configuration");

    // let rng = StdRng::from_os_rng();

    let maincfg = MainConfiguration::from_value(&yaml);
    // panic!("EMERGENCY EXIT - Testing out Main Configuration system. Here is the detected config. Use the V2 config due to changes.\n {maincfg:#?}");

    // let source = parse_source(&yaml);

    // let (source_kind, source_path) = match &source.source {
    //     SourceKind::File(path) => {
    //         println!("[...] - Source is file at path: {path}");
    //         ("file", path)
    //     }
    //     SourceKind::Url(url) => {
    //         println!("[...] - Source is at URL: {url}");
    //         ("url", url)
    //     }
    // };

    let out_path = maincfg.output.path;
    if !Path::new(&out_path.clone()).is_dir() {
        std::fs::create_dir_all(&out_path)?;
    }

    // let mut log = SystemLog::init(out_path.into())?;
    // log.header("APP INIT")?
    //     .sys_log("app started")?
    //     .begin_category("source")?
    //     .state_property("file", source_path)?
    //     .state_property("media-type", source_kind)?
    //     .state_property(
    //         "max-dim",
    //         source
    //             .max_dim
    //             .map(|n| n.to_string())
    //             .unwrap_or("<N/A>".into()),
    //     )?
    //     .end_category()?
    //     .begin_category("output")?
    //     .state_property("path", out_path)?;

    let iterations = maincfg.output.n as u64;

    // log.state_property("n", iterations.to_string())?;
    // log.end_category()?; // output

    println!(
        "{label_processing} | Processing image: {}",
        maincfg.source.kind.get_path()
    );
    println!(
        "      | ...with constraint: {}",
        maincfg.source.constraint_str()
    );

    let media = maincfg.source.perform();
    let dims = media.get_dimensions();

    println!("{label_info} | Image dimensions are: {dims:?}]");
    println!("      | Total pixels: {}", dims.0 * dims.1);

    println!("{label_alert} | Running {iterations} iterations...");

    // TODO: Add initial setup.

    let bar = ProgressBar::new(iterations);
    bar.set_style(
        ProgressStyle::with_template(
            "[{eta:>8.bold} remaining...] {pos:>4.dim}/{len:4} [{per_sec}] {bar:40.cyan/purple} ({percent:.bold}%) {msg}",
        )
        .unwrap(),
    );

    // log.header("EXECUTION")?;

    bar.enable_steady_tick(Duration::from_millis(100));

    (0..iterations).into_par_iter().for_each(|i| {
        bar.inc(1);

        // log.begin_category(format!("[{i}]"))?;

        match &media {
            ImageResult::Image(img) => {
                let effects = maincfg.effects.generate::<DynamicImage>();
                let mut image = img.clone();
                for effect in effects.iter() {
                    image = effect.affect(image);
                }

                image.save(format!("{out_path}/{i:<05}.png")).unwrap();
            }
            ImageResult::Gif(gif) => {
                let effects = maincfg.effects.generate::<Frame>();
                let frames = gif.clone();
                let frames_amnt = frames.len();
                let frames = frames
                    .into_par_iter()
                    .enumerate()
                    .map(|(i, mut frame)| {
                        bar.set_message(format!("frame {i} of {frames_amnt}"));
                        for effect in &effects {
                            frame = effect.affect(frame);
                        }
                        frame
                    })
                    .collect::<Vec<_>>();

                let file_out = File::create(format!("{out_path}/{i:<05}.gif")).unwrap();
                let mut encoder = GifEncoder::new(file_out);
                encoder
                    .set_repeat(image::codecs::gif::Repeat::Infinite)
                    .unwrap();
                encoder.encode_frames(frames.into_iter()).unwrap();
            }
            ImageResult::Anim(_anim) => {
                todo!("Not currently supported")
            }
        }

        // log.end_category()?;
        // log.newline()?;
    });

    let dur = bar.duration();
    let h = dur.as_secs() / (60 * 60);
    let m = dur.as_secs() / (60) % 60;
    let s = dur.as_secs() % 60;
    println!("done in {h:0>2}:{m:0>2}:{s:0>2}!");

    Ok(())
}
