use crate::arena::{DroplessArena, TypedArena};

#[test]
fn dropless_basic_allocation() {
  let mut arena = DroplessArena::with_chunk_size(64);
  let a_val = *arena.alloc(1u32);
  let b = arena.alloc(2u32);
  assert_eq!(a_val, 1);
  assert_eq!(*b, 2);

  let slice = arena.alloc_slice_copy(&[3u8, 4u8]);
  assert_eq!(slice, &mut [3u8, 4u8]);

  let text = arena.alloc_str("hi");
  assert_eq!(text, "hi");
}

#[test]
fn typed_arena_drop() {
  use std::rc::Rc;
  use std::cell::Cell;
  struct DropCounter(Rc<Cell<usize>>);
  impl Drop for DropCounter {
    fn drop(&mut self) {
      self.0.set(self.0.get() + 1);
    }
  }

  let counter = Rc::new(Cell::new(0usize));
  {
    let mut arena = TypedArena::<DropCounter>::new();
    arena.alloc(DropCounter(counter.clone()));
    arena.alloc(DropCounter(counter.clone()));
  }
  assert_eq!(counter.get(), 2);
}
