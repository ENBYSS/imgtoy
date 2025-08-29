use std::str::FromStr;

use image::{codecs::gif::GifDecoder, AnimationDecoder, DynamicImage, Frame, GenericImageView};
use indicatif::ProgressDrawTarget;
use mime::Mime;
use reqwest::header::{HeaderMap, CONTENT_TYPE};

use crate::parsers::v2::structure::meta::{Source, SourceKind};

#[derive(Clone, Copy)]
pub enum ImageKind {
    Image,
    Gif,
    Anim,
}

impl ImageKind {
    pub fn from_path(path: &str) -> Self {
        match path.split(".").collect::<Vec<&str>>().last() {
            Some(v) => Self::from_extension(v),
            None => todo!("extension is required"),
        }
    }

    pub fn from_headers(headers: &HeaderMap) -> Self {
        match headers.get(CONTENT_TYPE) {
            None => {
                todo!("handle case where no content type is present")
            }
            Some(content_type) => {
                let content_type = Mime::from_str(content_type.to_str().unwrap()).unwrap();
                Self::from_mime(content_type)
            }
        }
    }

    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "gif" => Self::Gif,
            "png" | "jpeg" | "jpg" | "tiff" | "bmp" => Self::Image,
            "mp4" | "mov" | "avi" => Self::Anim,
            _ => todo!("no support for extension [{ext}]"),
        }
    }

    pub fn from_mime(mime: Mime) -> Self {
        match (mime.type_(), mime.subtype()) {
            (mime::IMAGE, mime::GIF) => Self::Gif,
            (mime::VIDEO, _) => Self::Anim,
            (mime::IMAGE, _) => Self::Image,
            _ => todo!("no support for mime-type {mime}"),
        }
    }
}

#[derive(Clone)]
pub enum ImageResult {
    Image(DynamicImage),
    Anim(Vec<DynamicImage>),
    Gif(Vec<Frame>),
}

impl ImageResult {
    pub fn into_image(self) -> DynamicImage {
        if let ImageResult::Image(image) = self {
            image
        } else {
            todo!("oops not an image")
        }
    }

    pub fn into_gif(self) -> Vec<Frame> {
        if let ImageResult::Gif(gif) = self {
            gif
        } else {
            todo!("oops not a gif")
        }
    }

    pub fn into_anim(self) -> Vec<DynamicImage> {
        if let ImageResult::Anim(anim) = self {
            anim
        } else {
            todo!("oops not a gif")
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        match self {
            Self::Image(img) => img.dimensions(),
            Self::Anim(img) => img.get(0).unwrap().dimensions(),
            Self::Gif(gif) => {
                let frame = gif.get(0).unwrap();
                (frame.top(), frame.left())
            }
        }
    }
}

impl From<DynamicImage> for ImageResult {
    fn from(value: DynamicImage) -> Self {
        Self::Image(value)
    }
}

impl From<Vec<Frame>> for ImageResult {
    fn from(value: Vec<Frame>) -> Self {
        Self::Gif(value)
    }
}

pub struct ImageParser {}

impl ImageParser {
    pub fn parse_kind(source: &SourceKind) -> ImageKind {
        match source {
            SourceKind::File(path) => Self::parse_localkind(path),
            SourceKind::Url(path) => Self::parse_webkind(path),
        }
    }

    pub fn parse_file(source: &SourceKind) -> ImageResult {
        match source {
            SourceKind::File(path) => Self::parse_localfile(path),
            SourceKind::Url(path) => Self::parse_webfile(path),
        }
    }

    pub fn parse_localkind(path: &str) -> ImageKind {
        ImageKind::from_path(path)
    }

    pub fn parse_localfile(path: &str) -> ImageResult {
        let bytes = std::fs::read(path).unwrap();
        let res = Self::parse_bytes(&bytes, ImageKind::from_path(path));
        res
    }

    pub fn parse_webkind(url: &str) -> ImageKind {
        let response = reqwest::blocking::get(url).unwrap();
        ImageKind::from_headers(response.headers())
    }

    pub fn parse_webfile(url: &str) -> ImageResult {
        let response = reqwest::blocking::get(url).unwrap();
        let headers = response.headers().clone();
        let bytes = response.bytes().unwrap();

        let result = Self::parse_bytes(&bytes, ImageKind::from_headers(&headers));

        result
    }

    pub fn parse_bytes(bytes: &[u8], image_kind: ImageKind) -> ImageResult {
        let result = match image_kind {
            ImageKind::Gif => Self::gif_from_bytes(&bytes).into(),
            ImageKind::Image => Self::image_from_bytes(&bytes).into(),
            ImageKind::Anim => todo!("Anim currently unsupported."),
        };

        result
    }

    fn image_from_bytes(response: &[u8]) -> DynamicImage {
        image::load_from_memory(response).unwrap()
    }

    fn gif_from_bytes(response: &[u8]) -> Vec<Frame> {
        GifDecoder::new(response)
            .unwrap()
            .into_frames()
            .collect_frames()
            .unwrap()
    }
}
