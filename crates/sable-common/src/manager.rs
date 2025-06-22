use std::rc::Rc;

use crate::Source;

#[derive(Debug, Default)]
pub struct SourceManager {
    contents: Vec<Rc<Source>>, 
}

impl SourceManager {
    pub fn new() -> Self { Self { contents: Vec::new() } }

    pub fn add_content(&mut self, content: impl Into<String>, filename: impl Into<String>) -> Rc<Source> {
        let src = Rc::new(Source::new(content, filename));
        self.contents.push(src.clone());
        src
    }

    pub fn get_content(&self, filename: &str) -> Option<Rc<Source>> {
        self.contents.iter().find(|s| s.filename == filename).map(|s| s.clone())
    }
}
