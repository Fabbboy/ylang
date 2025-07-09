extern crate alloc;

use alloc::{
  boxed::Box,
  vec::Vec,
};
use core::{
  alloc::{
    AllocError,
    Allocator,
    Layout,
  },
  cell::RefCell,
  mem::{
    self,
    MaybeUninit,
  },
  ptr::{
    self,
    NonNull,
  },
  slice,
};

#[derive(Debug)]
struct Chunk {
  /// Pointer to the start of the allocation.
  storage: NonNull<u8>,
  /// Total size of the chunk.
  size: usize,
  /// Current bump position.
  pos: usize,
  /// Raw handle used to reconstruct the box on drop.
  raw: *mut [MaybeUninit<u8>],
}

#[cfg(feature = "serde")]
impl serde::Serialize for Chunk {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use serde::ser::SerializeStruct;
    let mut state = serializer.serialize_struct("Chunk", 2)?;
    state.serialize_field("size", &self.size)?;
    state.serialize_field("pos", &self.pos)?;
    state.end()
  }
}

impl Chunk {
  fn new(size: usize) -> Option<Self> {
    if size == 0 {
      return None;
    }
    let boxed = vec![MaybeUninit::<u8>::uninit(); size].into_boxed_slice();
    let raw = Box::into_raw(boxed);
    let storage = unsafe { (*raw).as_mut_ptr() as *mut u8 };
    Some(Self {
      storage: NonNull::new(storage)?,
      size,
      pos: 0,
      raw,
    })
  }

  fn alloc(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
    if size == 0 {
      return Some(NonNull::dangling());
    }
    let aligned = (self.pos + align - 1) & !(align - 1);
    if aligned + size > self.size {
      return None;
    }
    let ptr = unsafe { self.storage.as_ptr().add(aligned) };
    self.pos = aligned + size;
    Some(unsafe { NonNull::new_unchecked(ptr) })
  }

  fn try_retract(&mut self, ptr: NonNull<u8>, size: usize) -> bool {
    if size == 0 {
      return true;
    }
    let start = self.storage.as_ptr() as usize;
    let off = ptr.as_ptr() as usize - start;
    if off + size == self.pos {
      self.pos = off;
      true
    } else {
      false
    }
  }

  fn is_empty(&self) -> bool {
    self.pos == 0
  }
}

impl Drop for Chunk {
  fn drop(&mut self) {
    unsafe {
      drop(Box::from_raw(self.raw));
    }
  }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Arena {
  default_chunk_size: usize,
  chunks: RefCell<Vec<Chunk>>,
}

impl Arena {
  /// Default chunk size used when none is specified.
  pub const DEFAULT_CHUNK_SIZE: usize = 4096;

  /// Create a new empty arena using [`DEFAULT_CHUNK_SIZE`].
  pub fn new() -> Self {
    Self::with_chunk_size(Self::DEFAULT_CHUNK_SIZE)
  }

  /// Create a new arena with a custom default chunk size.
  pub fn with_chunk_size(chunk_size: usize) -> Self {
    Self {
      default_chunk_size: chunk_size,
      chunks: RefCell::new(Vec::new()),
    }
  }

  /// Allocate space for a value and write it to the arena.
  /// Panics if allocation fails.
  pub fn alloc<T>(&self, value: T) -> &mut T {
    self.try_alloc(value).expect("Arena allocation failed")
  }

  /// Try to allocate space for a value and write it to the arena.
  /// Returns None if allocation fails.
  pub fn try_alloc<T>(&self, value: T) -> Option<&mut T> {
    let layout = Layout::new::<T>();
    let ptr = self.try_alloc_raw(layout)?;
    unsafe {
      let typed = ptr.as_ptr() as *mut T;
      ptr::write(typed, value);
      Some(&mut *typed)
    }
  }

  /// Allocate a slice using a function to generate each element.
  /// Panics if allocation fails.
  pub fn alloc_slice_with<T>(&self, len: usize, f: impl FnMut(usize) -> T) -> &mut [T] {
    self
      .try_alloc_slice_with(len, f)
      .expect("Arena allocation failed")
  }

  /// Try to allocate a slice using a function to generate each element.
  /// Returns None if allocation fails.
  pub fn try_alloc_slice_with<T>(
    &self,
    len: usize,
    mut f: impl FnMut(usize) -> T,
  ) -> Option<&mut [T]> {
    if len == 0 {
      return Some(&mut []);
    }
    let layout = Layout::array::<T>(len).ok()?;
    let ptr = self.try_alloc_raw(layout)?;
    unsafe {
      let dst = ptr.as_ptr() as *mut T;
      for i in 0..len {
        dst.add(i).write(f(i));
      }
      Some(slice::from_raw_parts_mut(dst, len))
    }
  }

  /// Allocate a slice by copying from an existing slice.
  /// Panics if allocation fails.
  pub fn alloc_slice_copy<T: Copy>(&self, values: &[T]) -> &mut [T] {
    self
      .try_alloc_slice_copy(values)
      .expect("Arena allocation failed")
  }

  /// Try to allocate a slice by copying from an existing slice.
  /// Returns None if allocation fails.
  pub fn try_alloc_slice_copy<T: Copy>(&self, values: &[T]) -> Option<&mut [T]> {
    if values.is_empty() {
      return Some(&mut []);
    }
    let layout = Layout::array::<T>(values.len()).ok()?;
    let ptr = self.try_alloc_raw(layout)?;
    unsafe {
      let dst = ptr.as_ptr() as *mut T;
      ptr::copy_nonoverlapping(values.as_ptr(), dst, values.len());
      Some(slice::from_raw_parts_mut(dst, values.len()))
    }
  }

  /// Allocate a string by copying from an existing string.
  /// Panics if allocation fails.
  pub fn alloc_str(&self, string: &str) -> &mut str {
    self.try_alloc_str(string).expect("Arena allocation failed")
  }

  /// Try to allocate a string by copying from an existing string.
  /// Returns None if allocation fails.
  pub fn try_alloc_str(&self, string: &str) -> Option<&mut str> {
    if string.is_empty() {
      let empty = self.try_alloc_slice_copy(&[])?;
      return Some(unsafe { core::str::from_utf8_unchecked_mut(empty) });
    }
    let bytes = self.try_alloc_slice_copy(string.as_bytes())?;
    Some(unsafe { core::str::from_utf8_unchecked_mut(bytes) })
  }

  /// Allocate raw memory with the given layout.
  /// Panics if allocation fails.
  pub fn alloc_raw(&self, layout: Layout) -> NonNull<u8> {
    self.try_alloc_raw(layout).expect("Arena allocation failed")
  }

  /// Try to allocate raw memory with the given layout.
  /// Returns None if allocation fails.
  pub fn try_alloc_raw(&self, layout: Layout) -> Option<NonNull<u8>> {
    if layout.size() > isize::MAX as usize {
      return None;
    }
    let mut chunks = self.chunks.borrow_mut();
    for chunk in chunks.iter_mut() {
      if let Some(ptr) = chunk.alloc(layout.size(), layout.align()) {
        return Some(ptr);
      }
    }
    let needed_size = layout.size().max(self.default_chunk_size);
    // Grow the chunk to the next power of two so subsequent allocations
    // have room without immediately requiring another chunk.
    let alloc_size = needed_size
      .checked_next_power_of_two()
      .unwrap_or(needed_size)
      .min(isize::MAX as usize);
    if alloc_size < needed_size {
      return None;
    }
    let mut new_chunk = Chunk::new(alloc_size)?;
    let ptr = new_chunk.alloc(layout.size(), layout.align())?;
    chunks.push(new_chunk);
    Some(ptr)
  }

  fn dealloc_raw(&self, ptr: NonNull<u8>, layout: Layout) -> bool {
    if layout.size() == 0 {
      return true;
    }
    let mut chunks = self.chunks.borrow_mut();
    for chunk in chunks.iter_mut() {
      if chunk.try_retract(ptr, layout.size()) {
        return true;
      }
    }
    false
  }

  pub fn dealloc<T>(&self, ptr: &mut T) -> bool {
    let raw = NonNull::new(ptr as *mut T as *mut u8).unwrap();
    let size = mem::size_of::<T>();
    if size == 0 {
      return true;
    }
    self.dealloc_raw(raw, Layout::new::<T>())
  }

  pub fn dealloc_slice<T>(&self, slice: &mut [T]) -> bool {
    if slice.is_empty() {
      return true;
    }
    let raw = NonNull::new(slice.as_mut_ptr() as *mut u8).unwrap();
    let size = mem::size_of::<T>() * slice.len();
    self.dealloc_raw(
      raw,
      Layout::from_size_align(size, mem::align_of::<T>()).unwrap(),
    )
  }

  pub fn stats(&self) -> ArenaStats {
    let chunks = self.chunks.borrow();
    let mut total_size = 0;
    let mut total_used = 0;
    let mut empty_chunks = 0;
    for chunk in chunks.iter() {
      total_size += chunk.size;
      total_used += chunk.pos;
      if chunk.is_empty() {
        empty_chunks += 1;
      }
    }
    ArenaStats {
      total_chunks: chunks.len(),
      total_size,
      total_used,
      empty_chunks,
      default_chunk_size: self.default_chunk_size,
    }
  }

  pub fn clear(&self) {
    let mut chunks = self.chunks.borrow_mut();
    for chunk in chunks.iter_mut() {
      chunk.pos = 0;
    }
  }

  pub fn contains(&self, ptr: NonNull<u8>) -> bool {
    let chunks = self.chunks.borrow();
    chunks.iter().any(|chunk| {
      let start = chunk.storage.as_ptr() as usize;
      let end = start + chunk.size;
      let addr = ptr.as_ptr() as usize;
      addr >= start && addr < end
    })
  }

  pub fn compact(&self) {
    let mut chunks = self.chunks.borrow_mut();
    chunks.retain(|c| !c.is_empty());
  }
}

impl Default for Arena {
  fn default() -> Self {
    Self::new()
  }
}

unsafe impl Allocator for Arena {
  fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    self
      .try_alloc_raw(layout)
      .map(|ptr| NonNull::slice_from_raw_parts(ptr, layout.size()))
      .ok_or(AllocError)
  }

  unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
    let _ = self.dealloc_raw(ptr, layout);
  }
}

/// Information about arena memory usage.
#[derive(Debug)]
pub struct ArenaStats {
  pub total_chunks: usize,
  pub total_size: usize,
  pub total_used: usize,
  pub empty_chunks: usize,
  pub default_chunk_size: usize,
}

impl ArenaStats {
  pub fn utilization(&self) -> f32 {
    if self.total_size == 0 {
      0.0
    } else {
      self.total_used as f32 / self.total_size as f32
    }
  }

  pub fn average_chunk_utilization(&self) -> f32 {
    if self.total_chunks == 0 {
      0.0
    } else {
      self.utilization()
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::arena::Arena;

  #[test]
  fn test_basic_allocation() {
    let arena = Arena::with_chunk_size(1024);

    let ref1 = arena.alloc(42);
    let ref2 = arena.alloc(24);

    assert_eq!(*ref1, 42);
    assert_eq!(*ref2, 24);

    *ref1 = 100;
    assert_eq!(*ref1, 100);
  }

  #[test]
  fn test_slice_allocation_copy() {
    let arena = Arena::with_chunk_size(1024);

    let values = [1, 2, 3, 4, 5];
    let slice_ref = arena.alloc_slice_copy(&values);

    assert_eq!(slice_ref, &mut [1, 2, 3, 4, 5]);
    slice_ref[0] = 10;
    assert_eq!(slice_ref[0], 10);
  }

  #[test]
  fn test_slice_allocation_with_closure() {
    let arena = Arena::with_chunk_size(1024);

    let slice_ref = arena.alloc_slice_with(5, |i| i * 2);

    assert_eq!(slice_ref, &mut [0, 2, 4, 6, 8]);
    slice_ref[0] = 10;
    assert_eq!(slice_ref[0], 10);
  }

  #[test]
  fn test_string_allocation() {
    let arena = Arena::with_chunk_size(1024);

    let text = arena.alloc_str("Hello, world!");
    assert_eq!(text, "Hello, world!");

    let empty = arena.alloc_str("");
    assert_eq!(empty, "");
  }

  #[test]
  fn test_soft_cleanup() {
    let arena = Arena::with_chunk_size(1024);

    let ref1 = arena.alloc(42);
    let ref2 = arena.alloc(24);

    // This should not be retractable (not at end)
    assert!(!arena.dealloc(ref1));

    // This should be retractable (at end)
    assert!(arena.dealloc(ref2));

    // Now ref1 should be retractable
    assert!(arena.dealloc(ref1));
  }

  #[test]
  fn test_mixed_type_allocation() {
    let arena = Arena::with_chunk_size(1024);

    let int_ref = arena.alloc(42i32);
    let float_ref = arena.alloc(3.14f64);
    let string_ref = arena.try_alloc_str("test").unwrap();

    assert_eq!(*int_ref, 42);
    assert_eq!(*float_ref, 3.14);
    assert_eq!(string_ref, "test");

    // Test closure-based allocation with different types
    let int_slice = arena.alloc_slice_with(3, |i| (i as i32) * 10);
    assert_eq!(int_slice, &mut [0, 10, 20]);
  }

  #[test]
  fn test_zero_sized_types() {
    let arena = Arena::with_chunk_size(1024);

    #[derive(Debug, PartialEq)]
    struct ZeroSized;

    let ref1 = arena.alloc(ZeroSized);
    let ref2 = arena.alloc(ZeroSized);

    assert_eq!(*ref1, ZeroSized);
    assert_eq!(*ref2, ZeroSized);
  }

  #[test]
  fn test_large_allocation() {
    let arena = Arena::with_chunk_size(64); // Small chunks

    let large_array = [1u8; 128]; // Larger than chunk size
    let slice_ref = arena.alloc_slice_copy(&large_array);

    assert_eq!(slice_ref.len(), 128);
    assert_eq!(slice_ref[0], 1);
  }

  #[test]
  fn test_clear() {
    let arena = Arena::with_chunk_size(1024);

    let _ref1 = arena.alloc(42);
    let _ref2 = arena.alloc(24);

    let stats_before = arena.stats();
    assert!(stats_before.total_used > 0);

    arena.clear();

    let stats_after = arena.stats();
    assert_eq!(stats_after.total_used, 0);
  }

  #[test]
  fn test_non_copy_types_with_closure() {
    let arena = Arena::with_chunk_size(1024);

    // Test with non-Copy types
    let strings = arena.alloc_slice_with(3, |i| format!("item {}", i));
    assert_eq!(strings[0], "item 0");
    assert_eq!(strings[1], "item 1");
    assert_eq!(strings[2], "item 2");
  }

  #[test]
  fn test_std_allocator_api() {
    use std::{
      rc::Rc,
      sync::Arc,
    };

    let arena = Arena::new();

    let mut vec: Vec<u32, &Arena> = Vec::new_in(&arena);
    vec.extend([1, 2, 3]);
    assert_eq!(vec, [1, 2, 3]);

    let rc = Rc::new_in(5u32, &arena);
    assert_eq!(*rc, 5);

    let arc = Arc::new_in("hi".to_string(), &arena);
    assert_eq!(&*arc, "hi");
  }
}
