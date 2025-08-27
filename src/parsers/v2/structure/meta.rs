use serde_yaml::Value;

use crate::parsers::v2::structure::value::{parse_property_as_usize, Vusize};

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
    media_type: MediaType,
    max_dim: Option<Vusize>,
}

impl Source {
    pub fn from_value(value: &Value) -> Self {
        let source = value.get("source").unwrap();

        Self {
            kind: SourceKind::from_value(source),
            media_type: MediaType::from_value(source),
            max_dim: parse_property_as_usize(value, "max-dim"),
        }
    }
}

#[derive(Debug)]
pub struct Output {
    path: String,
    n: usize,
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
