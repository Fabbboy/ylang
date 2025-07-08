use std::alloc::Layout;
use std::mem::{self, MaybeUninit};

struct RawChunk {
    ptr: *mut MaybeUninit<u8>,
    cap: usize,
    pos: usize,
}

/// Basic arena using Box-backed chunks.
pub struct Arena {
    chunks: Vec<RawChunk>,
    chunk_size: usize,
}

impl Arena {
    pub fn new() -> Self {
        Self::with_chunk_size(4096)
    }

    pub fn with_chunk_size(chunk_size: usize) -> Self {
        Self { chunks: Vec::new(), chunk_size: chunk_size.max(1) }
    }

    #[inline]
    fn align_up(addr: usize, align: usize) -> usize {
        (addr + align - 1) & !(align - 1)
    }

    fn alloc_raw(&mut self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();

        if let Some(chunk) = self.chunks.last_mut() {
            let base = chunk.ptr as usize;
            let aligned = Self::align_up(base + chunk.pos, align);
            let end = aligned + size;
            if end <= base + chunk.cap {
                chunk.pos = end - base;
                return aligned as *mut u8;
            }
        }

        let needed = size + align;
        let mut cap = self.chunk_size.max(needed);
        if let Some(last) = self.chunks.last() {
            cap = cap.max(last.cap * 2);
        }
        let mut buf = vec![MaybeUninit::<u8>::uninit(); cap].into_boxed_slice();
        let ptr = buf.as_mut_ptr();
        let base = ptr as usize;
        let aligned = Self::align_up(base, align);
        let pos = aligned + size - base;
        let raw = Box::into_raw(buf) as *mut MaybeUninit<u8>;
        self.chunks.push(RawChunk { ptr: raw, cap, pos });
        aligned as *mut u8
    }

    pub fn clear(&mut self) {
        for chunk in self.chunks.drain(..) {
            unsafe {
                let slice = core::ptr::slice_from_raw_parts_mut(chunk.ptr, chunk.cap);
                drop(Box::from_raw(slice));
            }
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        self.clear();
    }
}

pub struct DroplessArena {
    arena: Arena,
}

impl DroplessArena {
  pub fn new() -> Self {
    Self::with_chunk_size(4096)
  }

  pub fn with_chunk_size(chunk_size: usize) -> Self {
    Self { arena: Arena::with_chunk_size(chunk_size) }
  }

  fn alloc_raw(&mut self, layout: Layout) -> *mut u8 {
    self.arena.alloc_raw(layout)
  }

  pub fn alloc<T>(&mut self, value: T) -> &mut T {
    assert!(!mem::needs_drop::<T>());
    let ptr = self.alloc_raw(Layout::new::<T>()) as *mut T;
    unsafe { ptr.write(value); &mut *ptr }
  }

  pub fn alloc_with<T>(&mut self, f: impl FnOnce() -> T) -> &mut T {
    self.alloc(f())
  }

  pub fn alloc_slice_copy<T: Copy>(&mut self, data: &[T]) -> &mut [T] {
    assert!(!mem::needs_drop::<T>());
    let layout = Layout::array::<T>(data.len()).unwrap();
    let ptr = self.alloc_raw(layout) as *mut T;
    unsafe {
      ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());
      std::slice::from_raw_parts_mut(ptr, data.len())
    }
  }

  pub fn alloc_slice_with<T>(&mut self, len: usize, mut f: impl FnMut(usize) -> T) -> &mut [T] {
    assert!(!mem::needs_drop::<T>());
    let layout = Layout::array::<T>(len).unwrap();
    let ptr = self.alloc_raw(layout) as *mut T;
    unsafe {
      for i in 0..len {
        ptr.add(i).write(f(i));
      }
      std::slice::from_raw_parts_mut(ptr, len)
    }
  }

  pub fn alloc_str(&mut self, s: &str) -> &mut str {
    let bytes = self.alloc_slice_copy::<u8>(s.as_bytes());
    unsafe { std::str::from_utf8_unchecked_mut(bytes) }
  }

  pub fn clear(&mut self) {
    self.arena.clear();
  }
}

impl Drop for DroplessArena {
  fn drop(&mut self) {
    self.arena.clear();
  }
}

pub struct TypedArena<T> {
  arena: DroplessArena,
  items: Vec<*mut T>,
}

impl<T> TypedArena<T> {
  pub fn new() -> Self {
    Self::with_chunk_size(4096)
  }

  pub fn with_chunk_size(chunk_size: usize) -> Self {
    Self { arena: DroplessArena::with_chunk_size(chunk_size), items: Vec::new() }
  }

  pub fn alloc(&mut self, value: T) -> &mut T {
    let ptr = self.arena.alloc_raw(Layout::new::<T>()) as *mut T;
    unsafe {
      ptr.write(value);
      self.items.push(ptr);
      &mut *ptr
    }
  }

  pub fn alloc_with(&mut self, f: impl FnOnce() -> T) -> &mut T {
    self.alloc(f())
  }

  pub fn clear(&mut self) {
    unsafe {
      for &ptr in &self.items {
        ptr.drop_in_place();
      }
    }
    self.items.clear();
    self.arena.clear();
  }
}

impl<T> Drop for TypedArena<T> {
  fn drop(&mut self) {
    unsafe {
      for &ptr in &self.items {
        ptr.drop_in_place();
      }
    }
  }
}
