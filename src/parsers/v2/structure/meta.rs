pub enum SourceKind {
    Url(String),
    File(String),
}

pub struct Source {
    kind: SourceKind,
    max_dim: Option<usize>,
}

pub struct Output {
    path: String,
    n: usize,
}
