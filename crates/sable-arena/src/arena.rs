extern crate alloc;

use alloc::{
  alloc::{
    AllocError,
    Allocator,
    Global,
  },
  collections::LinkedList,
};
use core::{
  alloc::Layout,
  mem::{
    self,
    MaybeUninit,
  },
  ptr::{
    self,
    NonNull,
  },
};

/// One arena chunk, with metadata at front
struct BufNode {
  size: usize,
}

pub struct Arena<A: Allocator = Global> {
  allocator: A,
  buffers: LinkedList<NonNull<BufNode>>,
  offset: usize,
}

impl Arena<Global> {
  pub fn new() -> Self {
    Self::with_allocator(Global)
  }
}

impl<A: Allocator + Clone> Arena<A> {
  pub fn with_allocator(allocator: A) -> Self {
    Self {
      allocator,
      buffers: LinkedList::new(),
      offset: 0,
    }
  }

  pub fn reset(&mut self) {
    while let Some(ptr) = self.buffers.pop_front() {
      unsafe {
        let size = ptr.as_ref().size;
        let layout = Layout::from_size_align_unchecked(size, 8);
        self.allocator.deallocate(ptr.cast(), layout);
      }
    }
    self.offset = 0;
  }

  #[inline(always)]
  fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
  }

  fn ensure_capacity(&mut self, layout: Layout) -> Result<*mut u8, AllocError> {
    let align = layout.align();
    let size = layout.size();

    if let Some(&ptr) = self.buffers.front() {
      unsafe {
        let base = (ptr.as_ptr() as *mut u8).add(mem::size_of::<BufNode>());
        let capacity = ptr.as_ref().size - mem::size_of::<BufNode>();

        let raw = base.add(self.offset);
        let aligned = Self::align_up(raw as usize, align);
        let end = aligned + size;

        if end <= base as usize + capacity {
          self.offset = end - base as usize;
          return Ok(aligned as *mut u8);
        }
      }
    }

    // Allocate a new buffer
    let alloc_size = (size + mem::size_of::<BufNode>()).max(1024);
    let layout = Layout::from_size_align(alloc_size, 8).map_err(|_| AllocError)?;
    let raw = self.allocator.allocate(layout)?;
    let buf = raw.as_ptr();

    unsafe {
      let buf_ptr = buf as *mut u8;
      buf_ptr
        .cast::<BufNode>()
        .write(BufNode { size: alloc_size });
      self
        .buffers
        .push_front(NonNull::new_unchecked(buf_ptr.cast()));
      let base = buf_ptr.add(mem::size_of::<BufNode>());
      let aligned = Self::align_up(base as usize, align);
      self.offset = aligned + size - base as usize;
      Ok(aligned as *mut u8)
    }
  }

  pub fn alloc<T>(&mut self, value: T) -> Result<&mut T, AllocError> {
    let layout = Layout::new::<T>();
    let ptr = self.ensure_capacity(layout)?;
    unsafe {
      let typed = ptr.cast::<T>();
      ptr::write(typed, value);
      Ok(&mut *typed)
    }
  }

  pub fn alloc_default<T: Default>(&mut self) -> Result<&mut T, AllocError> {
    self.alloc(T::default())
  }

  pub fn alloc_with<T>(&mut self, f: impl FnOnce() -> T) -> Result<&mut T, AllocError> {
    self.alloc(f())
  }

  pub fn alloc_slice_copy<T: Copy>(&mut self, data: &[T]) -> Result<&mut [T], AllocError> {
    let layout = Layout::array::<T>(data.len()).map_err(|_| AllocError)?;
    let ptr = self.ensure_capacity(layout)?;
    unsafe {
      let dst = ptr.cast::<T>();
      ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
      Ok(core::slice::from_raw_parts_mut(dst, data.len()))
    }
  }

  pub fn alloc_slice_fill<T>(
    &mut self,
    len: usize,
    mut f: impl FnMut(usize) -> T,
  ) -> Result<&mut [T], AllocError> {
    let layout = Layout::array::<T>(len).map_err(|_| AllocError)?;
    let ptr = self.ensure_capacity(layout)?;
    unsafe {
      let dst = ptr.cast::<T>();
      for i in 0..len {
        dst.add(i).write(f(i));
      }
      Ok(core::slice::from_raw_parts_mut(dst, len))
    }
  }

  fn free_all_buffers(&mut self) {
    while let Some(ptr) = self.buffers.pop_front() {
      unsafe {
        let size = ptr.as_ref().size;
        let layout = Layout::from_size_align_unchecked(size, 8);
        self.allocator.deallocate(ptr.cast(), layout);
      }
    }
    self.offset = 0;
  }
}

impl<A: Allocator> Drop for Arena<A> {
  fn drop(&mut self) {
    self.free_all_buffers();
  }
}
