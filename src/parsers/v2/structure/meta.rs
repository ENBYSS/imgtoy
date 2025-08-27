use std::error::Error;

use serde_yaml::Value;

use crate::{
    parsers::v2::structure::value::{parse_property_as_usize, Vusize},
    source::{parse_localfile, parse_webfile, resize_image_with_max_dim, ImageResult},
};

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
            todo!()
        } else if file.or(url).is_none() {
            todo!()
        }

        if let Some(file) = file {
            SourceKind::File(file.as_str().unwrap().to_string())
        } else if let Some(url) = url {
            SourceKind::Url(url.as_str().unwrap().to_string())
        } else {
            todo!()
        }
    }
}

#[derive(Debug)]
pub enum MediaType {
    Image,
    Gif,
}

impl MediaType {
    pub fn from_value(value: &Value) -> Self {
        let kind = value.get("media-type").unwrap().as_str().unwrap();

        match kind {
            "image" => MediaType::Image,
            "gif" => MediaType::Gif,
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Source {
    kind: SourceKind,
    // media_type: MediaType,
    max_dim: Option<Vusize>,
}

type UtilResult<T> = Result<T, Box<dyn Error>>;

impl Source {
    pub fn from_value(value: &Value) -> Self {
        let source = value.get("source").unwrap();

        Self {
            kind: SourceKind::from_value(source),
            // media_type: MediaType::from_value(source),
            max_dim: parse_property_as_usize(source, "max-dim"),
        }
    }

    pub fn perform(&self) -> UtilResult<ImageResult> {
        let result = match &self.kind {
            SourceKind::File(target) => {
                let file = parse_localfile(target)?;

                if self.max_dim.is_some() {
                    if let ImageResult::Image(image) = file {
                        resize_image_with_max_dim(&image, self.max_dim.as_ref().unwrap().generate())
                            .into()
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
                        resize_image_with_max_dim(&image, self.max_dim.as_ref().unwrap().generate())
                            .into()
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
