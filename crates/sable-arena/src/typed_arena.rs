use core::{
  alloc::{
    AllocError,
    Allocator,
    Layout,
  },
  marker::PhantomData,
  ptr::NonNull,
};

use super::arena::Arena;

#[derive(Debug)]
pub struct TypedArena<T> {
  inner: Arena,
  _marker: PhantomData<fn() -> T>,
}

impl<T> TypedArena<T> {
  pub fn new() -> Self {
    Self::with_chunk_size(Arena::DEFAULT_CHUNK_SIZE)
  }

  pub fn with_chunk_size(chunk_size: usize) -> Self {
    Self {
      inner: Arena::with_chunk_size(chunk_size),
      _marker: PhantomData,
    }
  }

  pub fn alloc(&self, value: T) -> &mut T {
    self.inner.alloc(value)
  }

  pub fn alloc_copy(&self, value: &T) -> &mut T {
    self.inner.alloc_copy(value)
  }

  pub fn alloc_slice_with(&self, len: usize, f: impl FnMut(usize) -> T) -> &mut [T] {
    self.inner.alloc_slice_with(len, f)
  }

  pub fn alloc_slice_default(&self, len: usize) -> &mut [T]
  where
    T: Default,
  {
    self.inner.alloc_slice_default(len)
  }

  pub fn alloc_slice_copy(&self, values: &[T]) -> &mut [T]
  where
    T: Copy,
  {
    self.inner.alloc_slice_copy(values)
  }

  pub fn alloc_str(&self, s: &str) -> &mut str {
    self.inner.alloc_str(s)
  }

  pub fn as_untyped(&self) -> &Arena {
    &self.inner
  }
}

impl<T> Default for TypedArena<T> {
  fn default() -> Self {
    Self::new()
  }
}

unsafe impl<T> Allocator for TypedArena<T> {
  fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    self.inner.allocate(layout)
  }

  unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
    unsafe { self.inner.deallocate(ptr, layout) }
  }

  unsafe fn grow(
    &self,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
  ) -> Result<NonNull<[u8]>, AllocError> {
    unsafe { self.inner.grow(ptr, old_layout, new_layout) }
  }

  unsafe fn shrink(
    &self,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
  ) -> Result<NonNull<[u8]>, AllocError> {
    unsafe { self.inner.shrink(ptr, old_layout, new_layout) }
  }
}
