use std::rc::Rc;
use std::ops::Range;

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

#[derive(Debug, Default)]
pub struct Manager {
    contents: Vec<Rc<Source>>,
}

impl Manager {
    pub fn new() -> Self {
        Self { contents: Vec::new() }
    }

    pub fn add_content(&mut self, content: impl Into<String>, filename: impl Into<String>) -> Rc<Source> {
        let src = Rc::new(Source::new(content, filename));
        self.contents.push(Rc::clone(&src));
        src
    }

    pub fn get_content(&self, filename: &str) -> Option<Rc<Source>> {
        self.contents.iter().find(|s| s.filename == filename).map(|s| Rc::clone(s))
    }
}

pub type RangeUsize = Range<usize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manager_add_and_get() {
        let mut mgr = Manager::new();
        let src = mgr.add_content("hello", "file.sable");
        let fetched = mgr.get_content("file.sable").unwrap();
        assert!(Rc::ptr_eq(&src, &fetched));
    }
}
