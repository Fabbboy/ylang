extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use core::{
    alloc::Layout,
    cell::RefCell,
    mem::{self, MaybeUninit},
    ptr::{self, NonNull},
    slice,
};

/// A chunk of raw memory backed by a leaked `Box`.
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
        unsafe { drop(Box::from_raw(self.raw)); }
    }
}

/// Untyped bump arena.
pub struct RawArena {
    default_chunk_size: usize,
    chunks: RefCell<Vec<Chunk>>,
}

impl RawArena {
    /// Default chunk size used when none is specified.
    pub const DEFAULT_CHUNK_SIZE: usize = 4096;

    /// Create a new empty arena using [`DEFAULT_CHUNK_SIZE`].
    pub fn new() -> Self {
        Self::with_chunk_size(Self::DEFAULT_CHUNK_SIZE)
    }

    /// Create a new arena with a custom default chunk size.
    pub fn with_chunk_size(chunk_size: usize) -> Self {
        Self { default_chunk_size: chunk_size, chunks: RefCell::new(Vec::new()) }
    }

    pub fn alloc<T>(&self, value: T) -> Option<&mut T> {
        let layout = Layout::new::<T>();
        let ptr = self.alloc_raw(layout)?;
        unsafe {
            let typed = ptr.as_ptr() as *mut T;
            ptr::write(typed, value);
            Some(&mut *typed)
        }
    }

    pub fn alloc_slice_with<T>(&self, len: usize, mut f: impl FnMut(usize) -> T) -> Option<&mut [T]> {
        if len == 0 {
            return Some(&mut []);
        }
        let layout = Layout::array::<T>(len).ok()?;
        let ptr = self.alloc_raw(layout)?;
        unsafe {
            let dst = ptr.as_ptr() as *mut T;
            for i in 0..len {
                dst.add(i).write(f(i));
            }
            Some(slice::from_raw_parts_mut(dst, len))
        }
    }

    pub fn alloc_slice_copy<T: Copy>(&self, values: &[T]) -> Option<&mut [T]> {
        if values.is_empty() {
            return Some(&mut []);
        }
        let layout = Layout::array::<T>(values.len()).ok()?;
        let ptr = self.alloc_raw(layout)?;
        unsafe {
            let dst = ptr.as_ptr() as *mut T;
            ptr::copy_nonoverlapping(values.as_ptr(), dst, values.len());
            Some(slice::from_raw_parts_mut(dst, values.len()))
        }
    }

    pub fn alloc_str(&self, string: &str) -> Option<&mut str> {
        if string.is_empty() {
            let empty = self.alloc_slice_copy(&[])?;
            return Some(unsafe { core::str::from_utf8_unchecked_mut(empty) });
        }
        let bytes = self.alloc_slice_copy(string.as_bytes())?;
        Some(unsafe { core::str::from_utf8_unchecked_mut(bytes) })
    }

    pub fn alloc_raw(&self, layout: Layout) -> Option<NonNull<u8>> {
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

    pub fn try_dealloc<T>(&self, ptr: &mut T) -> bool {
        let raw = NonNull::new(ptr as *mut T as *mut u8).unwrap();
        let size = mem::size_of::<T>();
        if size == 0 { return true; }
        let mut chunks = self.chunks.borrow_mut();
        for chunk in chunks.iter_mut() {
            if chunk.try_retract(raw, size) { return true; }
        }
        false
    }

    pub fn try_dealloc_slice<T>(&self, slice: &mut [T]) -> bool {
        if slice.is_empty() { return true; }
        let raw = NonNull::new(slice.as_mut_ptr() as *mut u8).unwrap();
        let size = mem::size_of::<T>() * slice.len();
        let mut chunks = self.chunks.borrow_mut();
        for chunk in chunks.iter_mut() {
            if chunk.try_retract(raw, size) { return true; }
        }
        false
    }

    pub fn stats(&self) -> ArenaStats {
        let chunks = self.chunks.borrow();
        let mut total_size = 0;
        let mut total_used = 0;
        let mut empty_chunks = 0;
        for chunk in chunks.iter() {
            total_size += chunk.size;
            total_used += chunk.pos;
            if chunk.is_empty() { empty_chunks += 1; }
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

impl Default for RawArena {
    fn default() -> Self {
        Self::new()
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
        if self.total_size == 0 { 0.0 } else { self.total_used as f32 / self.total_size as f32 }
    }

    pub fn average_chunk_utilization(&self) -> f32 {
        if self.total_chunks == 0 { 0.0 } else { self.utilization() }
    }
}

