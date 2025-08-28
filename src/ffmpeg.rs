use ez_ffmpeg::{FfmpegContext, FfmpegScheduler, Input, Output};

/// # Ideas
/// ## OpenGL
/// Look into using OpenGL for effects? Maybe generate shader code?
/// ## Indepth
/// Look deeper into ffmpeg commands?

pub struct FfmpegUtil {}

impl FfmpegUtil {
    pub fn demo() {
        Self::split_media_into_frames("https://images-ext-1.discordapp.net/external/7yNX5iM1y5yWlDVWngAIYqqozZxdci-gxuScOTZk028/https/media.tenor.com/KB7xapJWhPMAAAPo/ggg-great-god-grove.mp4", "demo");
        Self::combine_frames_into_gif("temp", "demo", "output/ffmpeg-demo.mp4");
        panic!("END OF FFMPEG DEMO");
    }

    // accepts gif, images, etc...
    pub fn split_media_into_frames(input: &str, temp_prefix: &str) {
        // ffmpeg -i input/Clap.gif -vsync 0 temp/temp%d.png
        let context = FfmpegContext::builder()
            .input(Input::from(input))
            .output(
                Output::from(format!("temp/{temp_prefix}-frame-%d.png")).set_vsync_method(
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

    pub fn combine_frames_into_gif(dir: &str, temp_prefix: &str, out: &str) {
        // ffmpeg -f image2 -framerate 9 -i image_%003d.jpg -vf scale=531x299,transpose=1,crop=299,431,0,100 out.gif
        let context = FfmpegContext::builder()
            .input(Input::from(format!("{dir}/{temp_prefix}-frame-%d.png")).set_format("image2"))
            // .filter_descs(vec![
            //     "scale=X.Y",        // change scale of output
            //     "transpose=1",      // rotate 90 degrees clockwise
            //     "crop=0,0,0,0",     // crop something
            // ])
            .output(Output::from(out))
            .build()
            .unwrap();

        FfmpegScheduler::new(context)
            .start()
            .unwrap()
            .wait()
            .unwrap();
    }
}
