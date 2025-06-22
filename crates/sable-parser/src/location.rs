use std::rc::Rc;
use sable_common::{RangeUsize, Source};

#[derive(Debug, Clone)]
pub struct Location {
    pub file: Rc<Source>,
    pub range: RangeUsize,
}

impl Location {
    pub fn new(file: Rc<Source>, range: RangeUsize) -> Self {
        Self { file, range }
    }
}
