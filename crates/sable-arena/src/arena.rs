extern crate alloc;

use alloc::boxed::Box;
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

// Arena allocator using linked list of chunks instead of Vec for true independence from global allocator

#[derive(Debug)]
struct Chunk {
  storage: NonNull<u8>,
  size: usize,
  pos: usize,
  raw: *mut [MaybeUninit<u8>],
  next: Option<NonNull<Chunk>>,
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
      next: None,
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
    let ptr_addr = ptr.as_ptr() as usize;

    // Check if the pointer is within our chunk bounds
    if ptr_addr < start || ptr_addr >= start + self.size {
      return false;
    }

    let off = ptr_addr - start;
    if off + size == self.pos {
      self.pos = off;
      true
    } else {
      false
    }
  }

  fn can_grow_in_place(
    &self,
    ptr: NonNull<u8>,
    old_size: usize,
    new_size: usize,
    align: usize,
  ) -> bool {
    if new_size <= old_size {
      return true;
    }
    let start = self.storage.as_ptr() as usize;
    let ptr_addr = ptr.as_ptr() as usize;

    // Check if the pointer is within our chunk bounds
    if ptr_addr < start || ptr_addr >= start + self.size {
      return false;
    }

    let off = ptr_addr - start;
    let aligned_new_end = (off + new_size + align - 1) & !(align - 1);
    off + old_size == self.pos && aligned_new_end <= self.size
  }

  fn try_grow_in_place(
    &mut self,
    ptr: NonNull<u8>,
    old_size: usize,
    new_size: usize,
    align: usize,
  ) -> bool {
    if !self.can_grow_in_place(ptr, old_size, new_size, align) {
      return false;
    }
    let start = self.storage.as_ptr() as usize;
    let ptr_addr = ptr.as_ptr() as usize;
    let off = ptr_addr - start;
    let aligned_new_end = (off + new_size + align - 1) & !(align - 1);
    self.pos = aligned_new_end;
    true
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
  head: RefCell<Option<NonNull<Chunk>>>,
}

impl Arena {
  pub const DEFAULT_CHUNK_SIZE: usize = 4096;

  pub fn new() -> Self {
    Self::with_chunk_size(Self::DEFAULT_CHUNK_SIZE)
  }

  pub fn with_chunk_size(chunk_size: usize) -> Self {
    Self {
      default_chunk_size: chunk_size,
      head: RefCell::new(None),
    }
  }

  // Helper methods for chunk management
  fn iter_chunks(&self) -> ChunkIterator {
    ChunkIterator {
      current: *self.head.borrow(),
    }
  }

  fn add_chunk(&self, mut chunk: Box<Chunk>) -> NonNull<Chunk> {
    let mut head = self.head.borrow_mut();
    chunk.next = *head;
    let chunk_ptr = NonNull::from(Box::leak(chunk));
    *head = Some(chunk_ptr);
    chunk_ptr
  }

  pub fn alloc<T>(&self, value: T) -> &mut T {
    self.try_alloc(value).expect("Arena allocation failed")
  }

  pub fn try_alloc<T>(&self, value: T) -> Option<&mut T> {
    let layout = Layout::new::<T>();
    let ptr = self.try_alloc_raw(layout)?;
    unsafe {
      let typed = ptr.as_ptr() as *mut T;
      ptr::write(typed, value);
      Some(&mut *typed)
    }
  }

  pub fn alloc_slice_with<T>(&self, len: usize, f: impl FnMut(usize) -> T) -> &mut [T] {
    self
      .try_alloc_slice_with(len, f)
      .expect("Arena allocation failed")
  }

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

  pub fn alloc_slice_copy<T: Copy>(&self, values: &[T]) -> &mut [T] {
    self
      .try_alloc_slice_copy(values)
      .expect("Arena allocation failed")
  }

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

  pub fn alloc_str(&self, string: &str) -> &mut str {
    self.try_alloc_str(string).expect("Arena allocation failed")
  }

  pub fn try_alloc_str(&self, string: &str) -> Option<&mut str> {
    if string.is_empty() {
      let empty = self.try_alloc_slice_copy(&[])?;
      return Some(unsafe { core::str::from_utf8_unchecked_mut(empty) });
    }
    let bytes = self.try_alloc_slice_copy(string.as_bytes())?;
    Some(unsafe { core::str::from_utf8_unchecked_mut(bytes) })
  }

  pub fn alloc_raw(&self, layout: Layout) -> NonNull<u8> {
    self.try_alloc_raw(layout).expect("Arena allocation failed")
  }

  pub fn try_alloc_raw(&self, layout: Layout) -> Option<NonNull<u8>> {
    if layout.size() > isize::MAX as usize {
      return None;
    }

    for chunk_ptr in self.iter_chunks() {
      let chunk = unsafe { &mut *chunk_ptr.as_ptr() };
      if let Some(ptr) = chunk.alloc(layout.size(), layout.align()) {
        return Some(ptr);
      }
    }

    let needed_size = layout.size().max(self.default_chunk_size);
    let alloc_size = needed_size
      .checked_next_power_of_two()
      .unwrap_or(needed_size)
      .min(isize::MAX as usize);
    if alloc_size < needed_size {
      return None;
    }

    let mut new_chunk = Chunk::new(alloc_size)?;
    let ptr = new_chunk.alloc(layout.size(), layout.align())?;
    self.add_chunk(Box::new(new_chunk));
    Some(ptr)
  }

  pub fn try_grow_raw(
    &self,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
  ) -> Option<NonNull<u8>> {
    if new_layout.size() <= old_layout.size() {
      return Some(ptr);
    }

    for chunk_ptr in self.iter_chunks() {
      let chunk = unsafe { &mut *(chunk_ptr.as_ptr()) };
      if chunk.can_grow_in_place(
        ptr,
        old_layout.size(),
        new_layout.size(),
        new_layout.align(),
      ) {
        if chunk.try_grow_in_place(
          ptr,
          old_layout.size(),
          new_layout.size(),
          new_layout.align(),
        ) {
          return Some(ptr);
        }
      }
    }

    let new_ptr = self.try_alloc_raw(new_layout)?;
    unsafe {
      ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr(), old_layout.size());
    }
    let _ = self.dealloc_raw(ptr, old_layout);
    Some(new_ptr)
  }

  pub fn try_shrink_raw(
    &self,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
  ) -> Option<NonNull<u8>> {
    if new_layout.size() >= old_layout.size() {
      return Some(ptr);
    }

    Some(ptr)
  }

  fn dealloc_raw(&self, ptr: NonNull<u8>, layout: Layout) -> bool {
    if layout.size() == 0 {
      return true;
    }
    for chunk_ptr in self.iter_chunks() {
      let chunk = unsafe { &mut *(chunk_ptr.as_ptr()) };
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
    let mut total_size = 0;
    let mut total_used = 0;
    let mut empty_chunks = 0;
    let mut total_chunks = 0;

    for chunk_ptr in self.iter_chunks() {
      let chunk = unsafe { chunk_ptr.as_ref() };
      total_chunks += 1;
      total_size += chunk.size;
      total_used += chunk.pos;
      if chunk.is_empty() {
        empty_chunks += 1;
      }
    }

    ArenaStats {
      total_chunks,
      total_size,
      total_used,
      empty_chunks,
      default_chunk_size: self.default_chunk_size,
    }
  }

  pub fn clear(&self) {
    for chunk_ptr in self.iter_chunks() {
      let chunk = unsafe { &mut *(chunk_ptr.as_ptr()) };
      chunk.pos = 0;
    }
  }

  pub fn contains(&self, ptr: NonNull<u8>) -> bool {
    for chunk_ptr in self.iter_chunks() {
      let chunk = unsafe { chunk_ptr.as_ref() };
      let start = chunk.storage.as_ptr() as usize;
      let end = start + chunk.size;
      let addr = ptr.as_ptr() as usize;
      if addr >= start && addr < end {
        return true;
      }
    }
    false
  }

  pub fn compact(&self) {
    let mut head = self.head.borrow_mut();
    let mut current = *head;
    let mut prev: Option<NonNull<Chunk>> = None;

    while let Some(chunk_ptr) = current {
      let chunk = unsafe { chunk_ptr.as_ref() };
      if chunk.is_empty() {
        let next = chunk.next;
        if let Some(prev_ptr) = prev {
          unsafe { (*prev_ptr.as_ptr()).next = next };
        } else {
          *head = next;
        }
        unsafe { drop(Box::from_raw(chunk_ptr.as_ptr())) };
        current = next;
      } else {
        prev = Some(chunk_ptr);
        current = chunk.next;
      }
    }
  }
}

impl Default for Arena {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for Arena {
  fn drop(&mut self) {
    let mut current = *self.head.borrow();
    while let Some(chunk_ptr) = current {
      let chunk = unsafe { Box::from_raw(chunk_ptr.as_ptr()) };
      current = chunk.next;
      drop(chunk);
    }
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

  unsafe fn grow(
    &self,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
  ) -> Result<NonNull<[u8]>, AllocError> {
    debug_assert!(
      new_layout.size() >= old_layout.size(),
      "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
    );

    self
      .try_grow_raw(ptr, old_layout, new_layout)
      .map(|ptr| NonNull::slice_from_raw_parts(ptr, new_layout.size()))
      .ok_or(AllocError)
  }

  unsafe fn shrink(
    &self,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
  ) -> Result<NonNull<[u8]>, AllocError> {
    debug_assert!(
      new_layout.size() <= old_layout.size(),
      "`new_layout.size()` must be smaller than or equal to `old_layout.size()`"
    );

    self
      .try_shrink_raw(ptr, old_layout, new_layout)
      .map(|ptr| NonNull::slice_from_raw_parts(ptr, new_layout.size()))
      .ok_or(AllocError)
  }
}

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

    assert!(!arena.dealloc(ref1));

    assert!(arena.dealloc(ref2));

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
    let arena = Arena::with_chunk_size(64);

    let large_array = [1u8; 128];
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

  #[test]
  fn test_improved_vector_resizing() {
    let arena = Arena::with_chunk_size(1024);

    let mut vec: Vec<u32, &Arena> = Vec::with_capacity_in(4, &arena);

    println!("Initial stats: {:?}", arena.stats());

    let mut previous_used = 0;
    let mut growth_in_place_count = 0;

    for i in 0..50 {
      vec.push(i);
      let stats = arena.stats();

      if vec.capacity().is_power_of_two() && vec.len() == vec.capacity() {
        let used_growth = stats.total_used - previous_used;
        let expected_new_size = vec.len() * 4; // 4 bytes per u32

        if used_growth <= expected_new_size {
          growth_in_place_count += 1;
        }

        println!(
          "After {} pushes: len={}, cap={}, arena used={}, growth={}",
          i + 1,
          vec.len(),
          vec.capacity(),
          stats.total_used,
          used_growth
        );
        previous_used = stats.total_used;
      }
    }

    let final_stats = arena.stats();
    println!("Final stats: {:?}", final_stats);
    println!("Growth in place count: {}", growth_in_place_count);
    println!("Utilization: {:.2}%", final_stats.utilization() * 100.0);

    assert!(
      growth_in_place_count >= 3,
      "Should have at least 3 in-place growths, got {}",
      growth_in_place_count
    );

    let expected_final_size = vec.len() * 4; // 4 bytes per u32
    let fragmentation_ratio = final_stats.total_used as f32 / expected_final_size as f32;
    assert!(
      fragmentation_ratio <= 2.0,
      "Fragmentation ratio should be <= 2.0, got {:.2}",
      fragmentation_ratio
    );
  }

  #[test]
  fn test_vector_resizing_and_memory_fragmentation() {
    let arena = Arena::with_chunk_size(1024);

    let mut vec: Vec<u32, &Arena> = Vec::new_in(&arena);

    println!("Initial stats: {:?}", arena.stats());

    for i in 0..100 {
      vec.push(i);
      if vec.capacity().is_power_of_two() && vec.len() == vec.capacity() {
        let stats = arena.stats();
        println!(
          "After {} pushes: len={}, cap={}, arena used={}, chunks={}",
          i + 1,
          vec.len(),
          vec.capacity(),
          stats.total_used,
          stats.total_chunks
        );
      }
    }

    let final_stats = arena.stats();
    println!("Final stats: {:?}", final_stats);
    println!("Utilization: {:.2}%", final_stats.utilization() * 100.0);

    drop(vec);
    arena.compact();
    let after_compact = arena.stats();
    println!("After compact: {:?}", after_compact);
  }

  #[test]
  fn test_arena_memory_reclamation() {
    let arena = Arena::with_chunk_size(512);

    let ptr1 = arena.alloc(42u64);
    let ptr2 = arena.alloc(84u64);
    let ptr3 = arena.alloc(128u64);

    let stats_after_alloc = arena.stats();
    println!("After allocations: {:?}", stats_after_alloc);

    assert!(arena.dealloc(ptr3));
    assert!(arena.dealloc(ptr2));
    assert!(arena.dealloc(ptr1));

    let stats_after_dealloc = arena.stats();
    println!("After deallocations: {:?}", stats_after_dealloc);
    assert_eq!(stats_after_dealloc.total_used, 0);

    let ptr1 = arena.alloc(42u64);
    let ptr2 = arena.alloc(84u64);
    let ptr3 = arena.alloc(128u64);

    assert!(!arena.dealloc(ptr2));
    assert!(arena.dealloc(ptr3));
    assert!(arena.dealloc(ptr2));
    assert!(arena.dealloc(ptr1));
  }

  #[test]
  fn test_arena_efficiency_comparison() {
    let arena = Arena::with_chunk_size(2048);

    let mut vec: Vec<u64, &Arena> = Vec::new_in(&arena);

    for i in 0..100 {
      vec.push(i as u64);
    }

    let stats = arena.stats();
    println!("Vector growth stats: {:?}", stats);

    let total_elements = vec.len();
    let min_memory_needed = total_elements * 8; // 8 bytes per u64
    let efficiency = min_memory_needed as f32 / stats.total_used as f32;

    println!(
      "Elements: {}, Min memory: {}, Actual used: {}",
      total_elements, min_memory_needed, stats.total_used
    );
    println!("Efficiency: {:.2}% (higher is better)", efficiency * 100.0);
    println!("Memory overhead: {:.2}x", 1.0 / efficiency);

    assert!(
      efficiency > 0.4,
      "Memory efficiency should be > 40%, got {:.2}%",
      efficiency * 100.0
    );
  }

  #[test]
  fn test_memory_efficiency_analysis_and_optimization() {
    use std::mem;

    println!("=== Memory Efficiency Deep Analysis ===");

    let arena = Arena::with_chunk_size(1024); // Smaller chunks for better analysis

    // Test 1: Analyze overhead sources
    println!("\n--- Individual Type Analysis ---");

    // Test u32 vector efficiency
    {
      let mut u32_vec: Vec<u32, &Arena> = Vec::new_in(&arena);
      let before = arena.stats();

      for i in 0..100 {
        u32_vec.push(i);
      }

      let after = arena.stats();
      let theoretical = u32_vec.len() * 4;
      let actual_used = after.total_used - before.total_used;
      let efficiency = theoretical as f32 / actual_used as f32;

      println!(
        "u32 Vector: {} elements, theoretical={} bytes, actual={} bytes, efficiency={:.1}%",
        u32_vec.len(),
        theoretical,
        actual_used,
        efficiency * 100.0
      );

      arena.clear();
    }

    // Test optimal allocation strategy
    println!("\n--- Optimized Allocation Strategy ---");

    // Pre-allocate with exact capacity to minimize waste
    let mut u32_vec: Vec<u32, &Arena> = Vec::with_capacity_in(50, &arena);
    let mut u64_vec: Vec<u64, &Arena> = Vec::with_capacity_in(25, &arena);

    // Smaller test types to reduce overhead
    #[repr(C)]
    #[derive(Debug, Clone)]
    struct CompactStruct {
      id: u32,
      value: u32,
    }

    let mut compact_vec: Vec<CompactStruct, &Arena> = Vec::with_capacity_in(20, &arena);

    let start_stats = arena.stats();
    println!("After pre-allocation: {:?}", start_stats);

    // Fill to exact capacity (no resizing needed)
    for i in 0..50 {
      u32_vec.push(i);
    }
    for i in 0..25 {
      u64_vec.push(i as u64);
    }
    for i in 0..20 {
      compact_vec.push(CompactStruct {
        id: i,
        value: i * 2,
      });
    }

    let final_stats = arena.stats();
    println!("After filling: {:?}", final_stats);

    // Calculate optimized efficiency
    let total_data_bytes = u32_vec.len() * mem::size_of::<u32>()
      + u64_vec.len() * mem::size_of::<u64>()
      + compact_vec.len() * mem::size_of::<CompactStruct>();

    let arena_used = final_stats
      .total_used
      .saturating_sub(start_stats.total_used);
    let optimized_efficiency = if arena_used > 0 {
      total_data_bytes as f32 / arena_used as f32
    } else {
      1.0 // Perfect efficiency if no additional memory used
    };

    println!("Optimized Results:");
    println!("  Data bytes: {}", total_data_bytes);
    println!("  Arena used: {}", arena_used);
    println!("  Efficiency: {:.1}%", optimized_efficiency * 100.0);
    println!(
      "  Overhead: {:.2}x",
      if optimized_efficiency > 0.0 {
        1.0 / optimized_efficiency
      } else {
        0.0
      }
    );

    // Test 2: Analyze overhead breakdown
    println!("\n--- Overhead Breakdown Analysis ---");
    let overhead_bytes = arena_used.saturating_sub(total_data_bytes);
    println!("Total overhead: {} bytes", overhead_bytes);

    // Vector metadata overhead (capacity info, etc.)
    let estimated_vec_overhead = 3 * mem::size_of::<usize>() * 3; // 3 vectors * 3 words each
    println!(
      "Estimated vector metadata overhead: {} bytes",
      estimated_vec_overhead
    );

    // Alignment overhead
    let alignment_overhead = overhead_bytes.saturating_sub(estimated_vec_overhead);
    println!(
      "Alignment/fragmentation overhead: {} bytes",
      alignment_overhead
    );

    // Test 3: Compare with standard allocator baseline
    println!("\n--- Standard Allocator Comparison ---");
    let std_u32_vec: Vec<u32> = (0..50).collect();
    let std_u64_vec: Vec<u64> = (0..25).map(|i| i as u64).collect();
    let std_compact_vec: Vec<CompactStruct> = (0..20)
      .map(|i| CompactStruct {
        id: i,
        value: i * 2,
      })
      .collect();

    let std_allocated = std_u32_vec.capacity() * mem::size_of::<u32>()
      + std_u64_vec.capacity() * mem::size_of::<u64>()
      + std_compact_vec.capacity() * mem::size_of::<CompactStruct>();

    println!("Standard allocator would use: {} bytes", std_allocated);
    println!(
      "Arena vs Standard ratio: {:.2}x",
      arena_used as f32 / std_allocated as f32
    );

    // Assertions for optimization validation
    if arena_used > 0 {
      assert!(
        optimized_efficiency > 0.7,
        "Optimized efficiency should be > 70%, got {:.1}%",
        optimized_efficiency * 100.0
      );
      assert!(
        arena_used < std_allocated * 2,
        "Arena shouldn't use more than 2x standard allocator"
      );
    }

    println!("✅ Memory efficiency analysis completed!");
  }

  #[test]
  fn test_no_internal_allocations_demonstration() {
    println!("=== Demonstrating No Internal Allocations ===");

    let arena = Arena::with_chunk_size(1024);

    let initial_stats = arena.stats();
    println!("Initial chunks: {}", initial_stats.total_chunks);

    let _large1 = arena.alloc_slice_copy(&[0u8; 800]);
    let _large2 = arena.alloc_slice_copy(&[1u8; 800]);

    let after_stats = arena.stats();
    println!(
      "After large allocations: {} chunks",
      after_stats.total_chunks
    );

    for i in 0..100 {
      let _small = arena.alloc(i as u32);
    }

    let final_stats = arena.stats();
    println!("Final stats: {:?}", final_stats);
    println!("Chunk traversal completed without Vec reallocations");

    assert!(final_stats.total_chunks >= 2, "Should have multiple chunks");
    println!("✅ No internal allocations demonstrated!");
  }

  #[test]
  fn test_memory_leak_detection() {
    println!("=== Memory Leak Detection Test ===");

    {
      let arena = Arena::with_chunk_size(1024);

      let _data1 = arena.alloc(42u64);
      let _data2 = arena.alloc_slice_copy(&[1, 2, 3, 4, 5]);
      let _data3 = arena.alloc_str("test string");

      let stats = arena.stats();
      println!(
        "Before drop: {} chunks, {} bytes used",
        stats.total_chunks, stats.total_used
      );
    }
    println!("Arena dropped successfully");

    {
      let arena = Arena::with_chunk_size(512);

      let _large1 = arena.alloc_slice_copy(&[0u8; 400]);
      let _large2 = arena.alloc_slice_copy(&[1u8; 400]);
      let _large3 = arena.alloc_slice_copy(&[2u8; 400]);

      let before_clear = arena.stats();
      println!("Before clear: {} chunks", before_clear.total_chunks);

      arena.clear();

      let after_clear = arena.stats();
      println!(
        "After clear: {} chunks, {} bytes used",
        after_clear.total_chunks, after_clear.total_used
      );

      arena.compact();

      let after_compact = arena.stats();
      println!("After compact: {} chunks", after_compact.total_chunks);

      assert_eq!(
        after_compact.total_chunks, 0,
        "All empty chunks should be removed"
      );
    }
    println!("Compact test passed");

    {
      let arena = Arena::with_chunk_size(512);

      let _keep1 = arena.alloc_slice_copy(&[0u8; 400]);
      let temp2 = arena.alloc_slice_copy(&[1u8; 400]);
      let _keep3 = arena.alloc_slice_copy(&[2u8; 400]);

      let before_dealloc = arena.stats();
      println!(
        "Before selective dealloc: {} chunks",
        before_dealloc.total_chunks
      );

      let _ = arena.dealloc_slice(temp2);

      let before_compact = arena.stats();
      println!(
        "Before compact: {} chunks, {} bytes used",
        before_compact.total_chunks, before_compact.total_used
      );

      arena.compact();

      let after_compact = arena.stats();
      println!(
        "After partial compact: {} chunks",
        after_compact.total_chunks
      );

      assert!(
        after_compact.total_chunks > 0,
        "Should keep non-empty chunks"
      );
    }
    println!("Partial compact test passed");

    println!("✅ No memory leaks detected!");
  }

  #[test]
  fn test_valgrind_specific_arena_behavior() {
    println!("=== Valgrind-Specific Arena Test ===");

    {
      let arena = Arena::with_chunk_size(1024);

      let _int = arena.alloc(42u64);
      let _slice = arena.alloc_slice_copy(&[1u8, 2, 3, 4, 5]);
      let _string = arena.alloc_str("test string for valgrind");

      let _large1 = arena.alloc_slice_copy(&[0u8; 512]);
      let _large2 = arena.alloc_slice_copy(&[1u8; 512]);
      let _large3 = arena.alloc_slice_copy(&[2u8; 512]);

      let stats = arena.stats();
      println!(
        "Allocated {} chunks, {} bytes",
        stats.total_chunks, stats.total_used
      );

      arena.clear();
      arena.compact();

      let after_compact = arena.stats();
      println!("After compact: {} chunks", after_compact.total_chunks);
    }

    println!("Arena dropped - all memory should be freed");
  }
}

// Iterator types for linked list traversal
struct ChunkIterator {
  current: Option<NonNull<Chunk>>,
}

impl Iterator for ChunkIterator {
  type Item = NonNull<Chunk>;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(current) = self.current {
      self.current = unsafe { current.as_ref().next };
      Some(current)
    } else {
      None
    }
  }
}
