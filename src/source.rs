use std::{error::Error, fs::File, io::Read, str::FromStr};

use image::{
    codecs::gif::GifDecoder, imageops, io::Reader, AnimationDecoder, DynamicImage, Frame,
    GenericImageView,
};
use mime::Mime;
use reqwest::{blocking::Response, header::CONTENT_TYPE};

#[derive(Clone)]
pub enum ImageResult {
    Image(DynamicImage),
    Gif(Vec<Frame>),
}

#[derive(Debug)]
pub struct Empty;

impl ImageResult {
    pub fn into_image(self) -> Result<DynamicImage, Empty> {
        if let ImageResult::Image(image) = self {
            Ok(image)
        } else {
            Err(Empty)
        }
    }

    pub fn into_gif(self) -> Result<Vec<Frame>, Empty> {
        if let ImageResult::Gif(gif) = self {
            Ok(gif)
        } else {
            Err(Empty)
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

#[derive(Debug, Clone)]
pub enum SourceKind {
    Url(String),
    File(String),
}

#[derive(Debug, Clone)]
pub enum MediaType {
    Image,
    Gif,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub source: SourceKind,
    // pub media_type: MediaType,
    pub max_dim: Option<usize>,
}

type UtilResult<T> = Result<T, Box<dyn Error>>;

impl Source {
    // #[deprecated = "replaced by function that auto-detects mime-type"]
    // pub fn _basic_perform(&self) -> UtilResult<ImageResult> {
    //     let result = match (&self.media_type, &self.source, &self.max_dim) {
    //         (MediaType::Image, SourceKind::File(target), None) => {
    //             load_image_from_path(target)?.into()
    //         }
    //         (MediaType::Image, SourceKind::File(target), Some(max_dim)) => {
    //             load_image_from_path_with_max_dim(target, *max_dim)?.into()
    //         }
    //         (MediaType::Image, SourceKind::Url(target), None) => {
    //             load_image_from_url(target)?.into()
    //         }
    //         (MediaType::Image, SourceKind::Url(target), Some(max_dim)) => {
    //             load_image_from_url_with_max_dim(target, *max_dim)?.into()
    //         }
    //         (MediaType::Gif, SourceKind::File(target), _) => load_gif_from_file(target)?.into(),
    //         (MediaType::Gif, SourceKind::Url(target), _) => load_gif_from_url(target)?.into(),
    //     };

    //     Ok(result)
    // }

    pub fn perform(&self) -> UtilResult<ImageResult> {
        let result = match &self.source {
            SourceKind::File(target) => {
                let file = parse_localfile(target)?;

                if self.max_dim.is_some() {
                    if let ImageResult::Image(image) = file {
                        resize_image_with_max_dim(&image, self.max_dim.unwrap()).into()
                    } else {
                        file
                    }
                } else {
                    file
                }
            }
            SourceKind::Url(target) => {
                let file = parse_webfile(target)?;

                if self.max_dim.is_some() {
                    if let ImageResult::Image(image) = file {
                        resize_image_with_max_dim(&image, self.max_dim.unwrap()).into()
                    } else {
                        file
                    }
                } else {
                    file
                }
            }
        };

        Ok(result)
    }
}

// resizers
pub fn resize_image(image: &DynamicImage, factor: f32) -> DynamicImage {
    let (x, y) = image.dimensions();
    let mul = |int: u32, float: f32| (int as f32 * float) as u32;
    image.resize(mul(x, factor), mul(y, factor), imageops::Nearest)
}

pub fn resize_image_with_max_dim(image: &DynamicImage, maxdim: usize) -> DynamicImage {
    let (x, y) = image.dimensions();
    if maxdim < x.max(y) as usize {
        resize_image(image, maxdim as f32 / x.max(y) as f32)
    } else {
        image.clone()
    }
}

// loaders

// image

pub fn parse_localfile(path: &str) -> UtilResult<ImageResult> {
    // let file = File::open(path).unwrap();
    // Ok(GifDecoder::new(file)?.into_frames().collect_frames()?)
    let bytes = std::fs::read(path).unwrap();

    let res = match path.split(".").collect::<Vec<&str>>().last() {
        Some(v) => match *v {
            "gif" => gif_from_bytes(&bytes)?.into(),
            "png" | "jpeg" | "jpg" | "tiff" => image_from_bytes(&bytes)?.into(),
            _ => todo!("handle unrecognized mimetype {v}"),
        },
        None => todo!("handle no extension"),
    };

    Ok(res)
}

pub fn parse_webfile(url: &str) -> UtilResult<ImageResult> {
    let response = reqwest::blocking::get(url)?;
    let headers = response.headers().clone();
    let bytes = response.bytes()?;

    let result = match headers.get(CONTENT_TYPE) {
        None => {
            todo!("handle case where no content type is present")
        }
        Some(content_type) => {
            let content_type = Mime::from_str(content_type.to_str()?)?;
            parse_bytes(&bytes, content_type)?
        }
    };

    Ok(result)
}

pub fn parse_bytes(bytes: &[u8], mime: Mime) -> UtilResult<ImageResult> {
    let result = match (mime.type_(), mime.subtype()) {
        (mime::IMAGE, mime::GIF) => gif_from_bytes(&bytes)?.into(),
        (mime::IMAGE, _) => image_from_bytes(&bytes)?.into(),
        (_, _) => todo!("handle unrecognized mimetype {mime}"),
    };

    Ok(result)
}

fn image_from_bytes(response: &[u8]) -> UtilResult<DynamicImage> {
    Ok(image::load_from_memory(response)?)
}

fn gif_from_bytes(response: &[u8]) -> UtilResult<Vec<Frame>> {
    Ok(GifDecoder::new(response)?.into_frames().collect_frames()?)
}

mod deprecated {
    use std::{fs::File, io::Read};

    use image::{codecs::gif::GifDecoder, io::Reader, AnimationDecoder, DynamicImage, Frame};

    use crate::source::{resize_image_with_max_dim, UtilResult};

    fn load_image_from_path(path: &str) -> UtilResult<DynamicImage> {
        Ok(Reader::open(path)?.decode()?)
    }

    fn load_image_from_path_with_max_dim(path: &str, maxdim: usize) -> UtilResult<DynamicImage> {
        let image = load_image_from_path(path)?;
        Ok(resize_image_with_max_dim(&image, maxdim))
    }

    fn load_image_from_url(url: &str) -> UtilResult<DynamicImage> {
        let img_bytes = reqwest::blocking::get(url)?.bytes()?;
        Ok(image::load_from_memory(&img_bytes)?)
    }

    fn load_image_from_url_with_max_dim(url: &str, maxdim: usize) -> UtilResult<DynamicImage> {
        let image = load_image_from_url(url)?;
        Ok(resize_image_with_max_dim(&image, maxdim))
    }

    // gif

    fn load_gif_from_file(path: &str) -> UtilResult<Vec<Frame>> {
        let file = File::open(path).unwrap();
        Ok(GifDecoder::new(file)?.into_frames().collect_frames()?)
    }

    fn load_gif_from_url(url: &str) -> UtilResult<Vec<Frame>> {
        let mut gif_bytes = reqwest::blocking::get(url)?;

        let mut data = Vec::new();
        gif_bytes.read_to_end(&mut data)?;

        Ok(GifDecoder::new(data.as_slice())?
            .into_frames()
            .collect_frames()?)
    }
}
