#[derive(Debug, Clone)]
pub struct Source {
    pub content: String,
    pub filename: String,
}

impl Source {
    pub fn new(content: impl Into<String>, filename: impl Into<String>) -> Self {
        Self { content: content.into(), filename: filename.into() }
    }
}
