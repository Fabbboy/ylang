use super::*;
use std::rc::Rc;

#[test]
fn manager_add_and_get() {
    let mut mgr = SourceManager::new();
    let src = mgr.add_content("hello", "file.sable");
    let fetched = mgr.get_content("file.sable").unwrap();
    assert!(Rc::ptr_eq(&src, &fetched));
}
