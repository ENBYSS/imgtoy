
use image::{imageops, DynamicImage, GenericImageView};
use regex::Regex;
use serde_yaml::Value;

use crate::utils::resource::image::parser::{ImageKind, ImageParser, ImageResult};

#[derive(Debug)]
pub enum SourceKind {
    Url(String),
    File(String),
}

impl SourceKind {
    pub fn from_value(value: &Value) -> Self {
        let file = value.get("file");
        let url = value.get("url");

        if file.and(url).is_some() {
            unimplemented!("only one of file/url accepted")
        } else if file.or(url).is_none() {
            unimplemented!("at least one of file/url required")
        }

        if let Some(file) = file {
            SourceKind::File(file.as_str().unwrap().to_string())
        } else if let Some(url) = url {
            let url = url.as_str().unwrap();
            let wre = Regex::new(r"&width=[0-9]+").unwrap();
            let hre = Regex::new(r"&height=[0-9]+").unwrap();
            let url = wre.replace_all(&url, "");
            let url = hre.replace_all(&url, "");
            SourceKind::Url(url.to_string())
        } else {
            unimplemented!("we shouldn't have even reached this point...")
        }
    }

    pub fn get_path(&self) -> String {
        match self {
            Self::File(path) => path.to_string(),
            Self::Url(path) => path.to_string(),
        }
    }

    pub fn get_image_kind(&self) -> ImageKind {
        match self {
            SourceKind::File(path) => ImageParser::parse_localkind(path),
            SourceKind::Url(path) => ImageParser::parse_webkind(path),
        }
    }
}

#[derive(Debug)]
pub enum SizeConstraint {
    MaxDim(usize),
    MaxPixels(usize),
}

impl SizeConstraint {
    pub fn from_value(value: &Value) -> Option<Self> {
        if let Some(value) = value.get("max-dim") {
            Some(Self::MaxDim(value.as_u64().unwrap() as usize))
        } else if let Some(value) = value.get("max-pixels") {
            Some(Self::MaxPixels(value.as_u64().unwrap() as usize))
        } else {
            None
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::MaxDim(n) => format!("max-dim: {n}"),
            Self::MaxPixels(n) => format!("max-pixels: {n}"),
        }
    }

    pub fn constrain(&self, image: DynamicImage) -> DynamicImage {
        let (x, y) = image.dimensions();

        match self {
            Self::MaxDim(max_dim) => {
                if *max_dim < x.max(y) as usize {
                    Self::resize_image(image, *max_dim as f32 / x.max(y) as f32)
                } else {
                    image
                }
            }
            Self::MaxPixels(max_pixels) => {
                let pixels = x * y;

                if pixels > *max_pixels as u32 {
                    Self::resize_image(image, (*max_pixels as f32 / pixels as f32).sqrt())
                } else {
                    image
                }
            }
        }
    }

    pub fn resize_image(image: DynamicImage, factor: f32) -> DynamicImage {
        let (x, y) = image.dimensions();
        let mul = |int: u32, float: f32| (int as f32 * float) as u32;
        image.resize(mul(x, factor), mul(y, factor), imageops::Nearest)
    }
}

#[derive(Debug)]
pub struct Source {
    pub kind: SourceKind,
    // media_type: MediaType,
    pub constraint: Option<SizeConstraint>,
}

impl Source {
    pub fn from_value(value: &Value) -> Self {
        let source = value.get("source").unwrap();

        Self {
            kind: SourceKind::from_value(source),
            // media_type: MediaType::from_value(source),
            constraint: SizeConstraint::from_value(source),
        }
    }

    pub fn constraint_str(&self) -> String {
        match &self.constraint {
            Some(max_dim) => max_dim.as_string(),
            None => "None".to_string(),
        }
    }

    pub fn perform(&self) -> ImageResult {
        let result = match &self.kind {
            SourceKind::File(target) => {
                let file = ImageParser::parse_localfile(target);

                if self.constraint.is_some() {
                    if let ImageResult::Image(image) = file {
                        self.constraint.as_ref().unwrap().constrain(image).into()
                    } else {
                        file
                    }
                } else {
                    file
                }
            }
            SourceKind::Url(target) => {
                let file = ImageParser::parse_webfile(&target);

                if self.constraint.is_some() {
                    if let ImageResult::Image(image) = file {
                        self.constraint.as_ref().unwrap().constrain(image).into()
                    } else {
                        file
                    }
                } else {
                    file
                }
            }
        };

        result
    }
}

#[derive(Debug)]
pub struct Output {
    pub path: String,
    pub n: usize,
}

impl Output {
    pub fn from_value(value: &Value) -> Self {
        let output = value.get("output").unwrap();

        Self {
            path: output.get("path").unwrap().as_str().unwrap().to_string(),
            n: output.get("n").unwrap().as_u64().unwrap() as usize,
        }
    }
}
