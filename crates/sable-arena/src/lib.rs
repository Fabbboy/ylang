#![no_std]

extern crate alloc;
use alloc::{
  alloc::{
    Layout,
    alloc,
    dealloc,
  },
  vec::Vec,
};
use core::{
  cell::RefCell,
  mem::{
    self,
  },
  ptr::{
    self,
    NonNull,
  },
  slice,
};

/// A chunk of raw memory with a bump allocator
struct Chunk {
  /// Raw memory for this chunk
  storage: NonNull<u8>,
  /// Total size of this chunk in bytes
  size: usize,
  /// Current position in the chunk (bump pointer)
  pos: usize,
}

impl Chunk {
  fn new(size: usize) -> Option<Self> {
    if size == 0 {
      return None;
    }

    let layout = Layout::from_size_align(size, 8).ok()?;
    let storage = unsafe { alloc(layout) };

    if storage.is_null() {
      return None;
    }

    Some(Self {
      storage: unsafe { NonNull::new_unchecked(storage) },
      size,
      pos: 0,
    })
  }

  /// Try to allocate `size` bytes with `align` alignment
  fn alloc(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
    if size == 0 {
      // Return a well-aligned non-null pointer for zero-sized allocations
      return Some(NonNull::new(align as *mut u8).unwrap_or(NonNull::dangling()));
    }

    // Calculate aligned position
    let aligned_pos = (self.pos + align - 1) & !(align - 1);

    if aligned_pos + size > self.size {
      return None;
    }

    let ptr = unsafe { self.storage.as_ptr().add(aligned_pos) };
    self.pos = aligned_pos + size;

    Some(unsafe { NonNull::new_unchecked(ptr) })
  }

  /// Try to retract the bump pointer if this allocation is at the end
  fn try_retract(&mut self, ptr: NonNull<u8>, size: usize) -> bool {
    if size == 0 {
      return true;
    }

    let ptr_offset = unsafe { ptr.as_ptr().offset_from(self.storage.as_ptr()) };
    if ptr_offset < 0 || ptr_offset as usize + size != self.pos {
      return false; // Not at the end
    }

    self.pos = ptr_offset as usize;
    true
  }

  fn is_empty(&self) -> bool {
    self.pos == 0
  }
}

impl Drop for Chunk {
  fn drop(&mut self) {
    let layout = Layout::from_size_align(self.size, 8).unwrap();
    unsafe {
      dealloc(self.storage.as_ptr(), layout);
    }
  }
}

/// The main untyped arena allocator
pub struct RawArena<const CHUNK_SIZE: usize = 4096> {
  /// Chunks of memory
  chunks: RefCell<Vec<Chunk>>,
}

impl<const CHUNK_SIZE: usize> RawArena<CHUNK_SIZE> {
  /// Create a new arena
  pub fn new() -> Self {
    Self {
      chunks: RefCell::new(Vec::new()),
    }
  }

  /// Allocate memory for a value of type T
  pub fn alloc<T>(&self, value: T) -> Option<&mut T> {
    let layout = Layout::new::<T>();
    let ptr = self.alloc_raw(layout)?;

    unsafe {
      let typed_ptr = ptr.as_ptr() as *mut T;
      ptr::write(typed_ptr, value);
      Some(&mut *typed_ptr)
    }
  }

  /// Allocate uninitialized memory for a slice of T and initialize it with a closure
  pub fn alloc_slice_with<T, F>(&self, len: usize, mut init: F) -> Option<&mut [T]>
  where
    F: FnMut(usize) -> T,
  {
    if len == 0 {
      return Some(&mut []);
    }

    let layout = Layout::array::<T>(len).ok()?;
    let ptr = self.alloc_raw(layout)?;

    unsafe {
      let typed_ptr = ptr.as_ptr() as *mut T;
      for i in 0..len {
        ptr::write(typed_ptr.add(i), init(i));
      }
      Some(slice::from_raw_parts_mut(typed_ptr, len))
    }
  }

  /// Allocate memory for a slice of Copy types (copying from source)
  pub fn alloc_slice_copy<T: Copy>(&self, values: &[T]) -> Option<&mut [T]> {
    if values.is_empty() {
      return Some(&mut []);
    }

    let layout = Layout::array::<T>(values.len()).ok()?;
    let ptr = self.alloc_raw(layout)?;

    unsafe {
      let typed_ptr = ptr.as_ptr() as *mut T;
      ptr::copy_nonoverlapping(values.as_ptr(), typed_ptr, values.len());
      Some(slice::from_raw_parts_mut(typed_ptr, values.len()))
    }
  }

  /// Allocate a string slice (copying from source)
  pub fn alloc_str(&self, string: &str) -> Option<&mut str> {
    if string.is_empty() {
      // Return an empty slice from arena memory instead of static ""
      let empty_bytes = self.alloc_slice_copy(&[])?;
      return Some(unsafe { core::str::from_utf8_unchecked_mut(empty_bytes) });
    }

    let bytes = self.alloc_slice_copy(string.as_bytes())?;
    Some(unsafe { core::str::from_utf8_unchecked_mut(bytes) })
  }

  /// Allocate raw memory with the given layout
  pub fn alloc_raw(&self, layout: Layout) -> Option<NonNull<u8>> {
    let mut chunks = self.chunks.borrow_mut();

    // Try to allocate in existing chunks
    for chunk in chunks.iter_mut() {
      if let Some(ptr) = chunk.alloc(layout.size(), layout.align()) {
        return Some(ptr);
      }
    }

    // Need a new chunk - make it big enough
    let needed_size = layout.size().max(CHUNK_SIZE);
    let mut new_chunk = Chunk::new(needed_size)?;
    let ptr = new_chunk.alloc(layout.size(), layout.align())?;

    chunks.push(new_chunk);
    Some(ptr)
  }

  /// Try to deallocate memory (soft cleanup)
  /// Returns true if the memory was at the end of a chunk and was retracted
  pub fn try_dealloc<T>(&self, ptr: &mut T) -> bool {
    let raw_ptr = NonNull::new(ptr as *mut T as *mut u8).unwrap();
    let size = mem::size_of::<T>();

    if size == 0 {
      return true;
    }

    let mut chunks = self.chunks.borrow_mut();
    for chunk in chunks.iter_mut() {
      if chunk.try_retract(raw_ptr, size) {
        return true;
      }
    }

    false
  }

  /// Try to deallocate a slice (soft cleanup)
  pub fn try_dealloc_slice<T>(&self, slice: &mut [T]) -> bool {
    if slice.is_empty() {
      return true;
    }

    let raw_ptr = NonNull::new(slice.as_mut_ptr() as *mut u8).unwrap();
    let size = mem::size_of::<T>() * slice.len();

    let mut chunks = self.chunks.borrow_mut();
    for chunk in chunks.iter_mut() {
      if chunk.try_retract(raw_ptr, size) {
        return true;
      }
    }

    false
  }

  /// Get statistics about arena usage
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
      chunk_size: CHUNK_SIZE,
    }
  }

  /// Clear all allocations (reset bump pointers)
  pub fn clear(&self) {
    let mut chunks = self.chunks.borrow_mut();
    for chunk in chunks.iter_mut() {
      chunk.pos = 0;
    }
  }
}

pub type Arena = RawArena<4096>;

impl<const CHUNK_SIZE: usize> Default for RawArena<CHUNK_SIZE> {
  fn default() -> Self {
    Self::new()
  }
}

/// Statistics about arena memory usage
#[derive(Debug)]
pub struct ArenaStats {
  pub total_chunks: usize,
  pub total_size: usize,
  pub total_used: usize,
  pub empty_chunks: usize,
  pub chunk_size: usize,
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
  use alloc::format;

  use super::*;

  #[test]
  fn test_basic_allocation() {
    let arena = RawArena::<1024>::new();

    let ref1 = arena.alloc(42).unwrap();
    let ref2 = arena.alloc(24).unwrap();

    assert_eq!(*ref1, 42);
    assert_eq!(*ref2, 24);

    *ref1 = 100;
    assert_eq!(*ref1, 100);
  }

  #[test]
  fn test_slice_allocation_copy() {
    let arena = RawArena::<1024>::new();

    let values = [1, 2, 3, 4, 5];
    let slice_ref = arena.alloc_slice_copy(&values).unwrap();

    assert_eq!(slice_ref, &mut [1, 2, 3, 4, 5]);
    slice_ref[0] = 10;
    assert_eq!(slice_ref[0], 10);
  }

  #[test]
  fn test_slice_allocation_with_closure() {
    let arena = RawArena::<1024>::new();

    let slice_ref = arena.alloc_slice_with(5, |i| i * 2).unwrap();

    assert_eq!(slice_ref, &mut [0, 2, 4, 6, 8]);
    slice_ref[0] = 10;
    assert_eq!(slice_ref[0], 10);
  }

  #[test]
  fn test_string_allocation() {
    let arena = RawArena::<1024>::new();

    let text = arena.alloc_str("Hello, world!").unwrap();
    assert_eq!(text, "Hello, world!");

    let empty = arena.alloc_str("").unwrap();
    assert_eq!(empty, "");
  }

  #[test]
  fn test_soft_cleanup() {
    let arena = RawArena::<1024>::new();

    let ref1 = arena.alloc(42).unwrap();
    let ref2 = arena.alloc(24).unwrap();

    // This should not be retractable (not at end)
    assert!(!arena.try_dealloc(ref1));

    // This should be retractable (at end)
    assert!(arena.try_dealloc(ref2));

    // Now ref1 should be retractable
    assert!(arena.try_dealloc(ref1));
  }

  #[test]
  fn test_mixed_type_allocation() {
    let arena = RawArena::<1024>::new();

    let int_ref = arena.alloc(42i32).unwrap();
    let float_ref = arena.alloc(3.14f64).unwrap();
    let string_ref = arena.alloc_str("test").unwrap();

    assert_eq!(*int_ref, 42);
    assert_eq!(*float_ref, 3.14);
    assert_eq!(string_ref, "test");

    // Test closure-based allocation with different types
    let int_slice = arena.alloc_slice_with(3, |i| (i as i32) * 10).unwrap();
    assert_eq!(int_slice, &mut [0, 10, 20]);
  }

  #[test]
  fn test_zero_sized_types() {
    let arena = RawArena::<1024>::new();

    #[derive(Debug, PartialEq)]
    struct ZeroSized;

    let ref1 = arena.alloc(ZeroSized).unwrap();
    let ref2 = arena.alloc(ZeroSized).unwrap();

    assert_eq!(*ref1, ZeroSized);
    assert_eq!(*ref2, ZeroSized);
  }

  #[test]
  fn test_large_allocation() {
    let arena = RawArena::<64>::new(); // Small chunks

    let large_array = [1u8; 128]; // Larger than chunk size
    let slice_ref = arena.alloc_slice_copy(&large_array).unwrap();

    assert_eq!(slice_ref.len(), 128);
    assert_eq!(slice_ref[0], 1);
  }

  #[test]
  fn test_clear() {
    let arena = RawArena::<1024>::new();

    let _ref1 = arena.alloc(42).unwrap();
    let _ref2 = arena.alloc(24).unwrap();

    let stats_before = arena.stats();
    assert!(stats_before.total_used > 0);

    arena.clear();

    let stats_after = arena.stats();
    assert_eq!(stats_after.total_used, 0);
  }

  #[test]
  fn test_non_copy_types_with_closure() {
    let arena = RawArena::<1024>::new();

    // Test with non-Copy types
    let strings = arena
      .alloc_slice_with(3, |i| format!("item {}", i))
      .unwrap();
    assert_eq!(strings[0], "item 0");
    assert_eq!(strings[1], "item 1");
    assert_eq!(strings[2], "item 2");
  }
}
