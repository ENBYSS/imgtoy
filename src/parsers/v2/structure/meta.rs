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

    pub fn get_path(&self) -> String {
        match self {
            Self::File(path) => path.to_string(),
            Self::Url(path) => path.to_string(),
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
    pub kind: SourceKind,
    // media_type: MediaType,
    pub max_dim: Option<usize>,
}

type UtilResult<T> = Result<T, Box<dyn Error>>;

impl Source {
    pub fn from_value(value: &Value) -> Self {
        let source = value.get("source").unwrap();

        Self {
            kind: SourceKind::from_value(source),
            // media_type: MediaType::from_value(source),
            max_dim: source.get("max-dim").map(|m| m.as_u64().unwrap() as usize),
        }
    }

    pub fn max_dim_str(&self) -> String {
        match self.max_dim {
            Some(max_dim) => format!("{max_dim}"),
            None => "None".to_string(),
        }
    }

    pub fn perform(&self) -> UtilResult<ImageResult> {
        let result = match &self.kind {
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
