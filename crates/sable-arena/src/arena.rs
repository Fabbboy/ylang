extern crate alloc;
use alloc::{
  alloc::{
    Allocator,
    Global,
    Layout,
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
struct Chunk<A: Allocator> {
  /// Raw memory for this chunk
  storage: NonNull<u8>,
  /// Total size of this chunk in bytes
  size: usize,
  /// Current position in the chunk (bump pointer)
  pos: usize,
  /// The allocator used for this chunk
  allocator: A,
}

impl<A: Allocator> Chunk<A> {
  fn new(size: usize, allocator: A) -> Result<Self, ()> {
    if size == 0 {
      return Err(());
    }

    let layout = Layout::from_size_align(size, 8).map_err(|_| ())?;
    let storage = allocator.allocate(layout).map_err(|_| ())?;

    Ok(Self {
      storage: storage.cast(),
      size,
      pos: 0,
      allocator,
    })
  }

  /// Try to allocate `size` bytes with `align` alignment
  fn alloc(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
    if size == 0 {
      // Return a well-aligned non-null pointer for zero-sized allocations
      return Some(
        NonNull::new(core::cmp::max(align, 1) as *mut u8).unwrap_or(NonNull::dangling()),
      );
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

impl<A: Allocator> Drop for Chunk<A> {
  fn drop(&mut self) {
    let layout = Layout::from_size_align(self.size, 8).unwrap();
    unsafe {
      self.allocator.deallocate(self.storage.cast(), layout);
    }
  }
}

/// The main untyped arena allocator
pub struct RawArena<const CHUNK_SIZE: usize = 4096, A: Allocator = Global> {
  /// Chunks of memory
  chunks: RefCell<Vec<Chunk<A>>>,
  /// The allocator to use for new chunks
  allocator: A,
}

impl<const CHUNK_SIZE: usize, A: Allocator + Clone> RawArena<CHUNK_SIZE, A> {
  /// Create a new arena with the specified allocator
  pub fn new_in(allocator: A) -> Self {
    Self {
      chunks: RefCell::new(Vec::new()),
      allocator,
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
    // Validate layout isn't too large
    if layout.size() > isize::MAX as usize {
      return None;
    }

    let mut chunks = self.chunks.borrow_mut();

    // Try to allocate in existing chunks
    for chunk in chunks.iter_mut() {
      if let Some(ptr) = chunk.alloc(layout.size(), layout.align()) {
        return Some(ptr);
      }
    }

    // Need a new chunk - make it big enough
    let needed_size = layout.size().max(CHUNK_SIZE);
    let mut new_chunk = Chunk::new(needed_size, self.allocator.clone()).ok()?;
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

  /// Check if a pointer was allocated by this arena
  pub fn contains(&self, ptr: NonNull<u8>) -> bool {
    let chunks = self.chunks.borrow();
    chunks.iter().any(|chunk| {
      let start = chunk.storage.as_ptr() as usize;
      let end = start + chunk.size;
      let ptr_addr = ptr.as_ptr() as usize;
      ptr_addr >= start && ptr_addr < end
    })
  }

  /// Compact the arena by removing empty chunks
  pub fn compact(&self) {
    let mut chunks = self.chunks.borrow_mut();
    chunks.retain(|chunk| !chunk.is_empty());
  }
}

// Convenience implementations for Global allocator
impl<const CHUNK_SIZE: usize> RawArena<CHUNK_SIZE, Global> {
  /// Create a new arena with the global allocator
  pub fn new() -> Self {
    Self::new_in(Global)
  }
}

impl<const CHUNK_SIZE: usize> Default for RawArena<CHUNK_SIZE, Global> {
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
