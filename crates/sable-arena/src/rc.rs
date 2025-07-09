use std::{
  borrow::Borrow,
  cell::Cell,
  cmp::Ordering,
  fmt::{
    self,
    Debug,
    Display,
    Formatter,
  },
  hash::{
    Hash,
    Hasher,
  },
  ops::Deref,
  ptr::NonNull,
};

use crate::arena::Arena;

struct ArenaRcInner<T> {
  strong: Cell<usize>,
  data: T,
}

/// A single-threaded reference-counted pointer backed by an arena.
///
/// This is similar to `std::rc::Rc` but allocates the inner data
/// in the provided arena instead of on the heap.
pub struct ArenaRc<'a, T> {
  ptr: NonNull<ArenaRcInner<T>>,
  _arena: &'a Arena,
}

impl<'a, T> ArenaRc<'a, T> {
  /// Constructs a new `ArenaRc<T>` in the given arena.
  pub fn new(data: T, arena: &'a Arena) -> Self {
    let inner = arena.alloc(ArenaRcInner {
      strong: Cell::new(1),
      data,
    });

    Self {
      ptr: NonNull::from(inner),
      _arena: arena,
    }
  }

  /// Returns the number of strong references to this allocation.
  pub fn strong_count(&self) -> usize {
    unsafe { self.ptr.as_ref().strong.get() }
  }

  /// Returns `true` if there are no other `ArenaRc` pointers to this allocation.
  pub fn is_unique(&self) -> bool {
    self.strong_count() == 1
  }

  /// Gets a mutable reference to the inner value if this `ArenaRc` is unique.
  pub fn get_mut(&mut self) -> Option<&mut T> {
    if self.is_unique() {
      // SAFETY: We have exclusive access to the ArenaRc and there's only one reference
      unsafe { Some(&mut self.ptr.as_mut().data) }
    } else {
      None
    }
  }

  /// Returns a raw pointer to the inner data.
  pub fn as_ptr(&self) -> *const T {
    unsafe { &self.ptr.as_ref().data as *const T }
  }

  /// Creates a new `ArenaRc` from a raw pointer.
  ///
  /// # Safety
  /// The raw pointer must have been returned by `into_raw` and the arena
  /// must still be valid.
  pub unsafe fn from_raw(ptr: *const T, arena: &'a Arena) -> Self {
    // Calculate the offset to the ArenaRcInner from the data field
    unsafe {
      let dummy = std::ptr::null::<ArenaRcInner<T>>();
      let data_offset = std::ptr::addr_of!((*dummy).data) as usize;
      let inner_ptr = (ptr as *const u8).sub(data_offset) as *mut ArenaRcInner<T>;

      Self {
        ptr: NonNull::new_unchecked(inner_ptr),
        _arena: arena,
      }
    }
  }

  /// Consumes the `ArenaRc`, returning the wrapped pointer.
  pub fn into_raw(this: Self) -> *const T {
    let ptr = this.as_ptr();
    std::mem::forget(this);
    ptr
  }

  /// Provides a raw pointer to the data.
  ///
  /// This is equivalent to `ArenaRc::as_ptr`, but can be called on temporaries.
  pub fn ptr_eq(this: &Self, other: &Self) -> bool {
    this.ptr == other.ptr
  }

  /// Returns the inner value, if this `ArenaRc` has exactly one reference.
  /// Otherwise, returns `Err(self)`.
  pub fn try_unwrap(this: Self) -> Result<T, Self> {
    if this.strong_count() == 1 {
      let ptr = this.ptr;
      std::mem::forget(this);
      // SAFETY: We're the only reference, so we can take ownership
      unsafe {
        let inner = ptr.as_ref();
        Ok(std::ptr::read(&inner.data))
      }
    } else {
      Err(this)
    }
  }
}

impl<'a, T: Default> ArenaRc<'a, T> {
  /// Creates a new `ArenaRc<T>` with the default value of `T` in the given arena.
  pub fn new_default(arena: &'a Arena) -> Self {
    Self::new(T::default(), arena)
  }
}

impl<'a, T> Clone for ArenaRc<'a, T> {
  fn clone(&self) -> Self {
    let old_size = unsafe {
      let inner = self.ptr.as_ref();
      let old_count = inner.strong.get();
      inner.strong.set(old_count + 1);
      old_count
    };

    // Check for overflow
    if old_size > (isize::MAX as usize) / 2 {
      panic!("ArenaRc reference count overflow");
    }

    Self {
      ptr: self.ptr,
      _arena: self._arena,
    }
  }
}

impl<'a, T> Deref for ArenaRc<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &self.ptr.as_ref().data }
  }
}

impl<'a, T> Drop for ArenaRc<'a, T> {
  fn drop(&mut self) {
    let old_count = unsafe {
      let inner = self.ptr.as_ref();
      let old_count = inner.strong.get();
      inner.strong.set(old_count - 1);
      old_count
    };

    if old_count == 1 {
      // This was the last reference, run the destructor
      unsafe {
        std::ptr::drop_in_place(&mut self.ptr.as_mut().data);
      }
      // Try to deallocate from the arena - if this allocation is at the end
      // of the bump allocator, the arena can reclaim the memory
      unsafe {
        let inner_ptr = self.ptr.as_mut();
        self._arena.dealloc(inner_ptr);
      }
    }
  }
}

impl<'a, T> AsRef<T> for ArenaRc<'a, T> {
  fn as_ref(&self) -> &T {
    &**self
  }
}

impl<'a, T> Borrow<T> for ArenaRc<'a, T> {
  fn borrow(&self) -> &T {
    &**self
  }
}

impl<'a, T: Debug> Debug for ArenaRc<'a, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Debug::fmt(&**self, f)
  }
}

impl<'a, T: Display> Display for ArenaRc<'a, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Display::fmt(&**self, f)
  }
}

impl<'a, T: PartialEq> PartialEq for ArenaRc<'a, T> {
  fn eq(&self, other: &Self) -> bool {
    **self == **other
  }
}

impl<'a, T: Eq> Eq for ArenaRc<'a, T> {}

impl<'a, T: PartialOrd> PartialOrd for ArenaRc<'a, T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    (**self).partial_cmp(&**other)
  }
}

impl<'a, T: Ord> Ord for ArenaRc<'a, T> {
  fn cmp(&self, other: &Self) -> Ordering {
    (**self).cmp(&**other)
  }
}

impl<'a, T: Hash> Hash for ArenaRc<'a, T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    (**self).hash(state)
  }
}

// ArenaRc is not Send or Sync (single-threaded only)
// This is the same behavior as std::rc::Rc

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_functionality() {
    let arena = Arena::new();
    let rc = ArenaRc::new(42, &arena);
    assert_eq!(*rc, 42);
    assert_eq!(rc.strong_count(), 1);
    assert!(rc.is_unique());
  }

  #[test]
  fn test_clone_and_reference_counting() {
    let arena = Arena::new();
    let rc1 = ArenaRc::new(String::from("hello"), &arena);
    assert_eq!(rc1.strong_count(), 1);

    let rc2 = rc1.clone();
    assert_eq!(rc1.strong_count(), 2);
    assert_eq!(rc2.strong_count(), 2);
    assert_eq!(*rc1, *rc2);
    assert!(!rc1.is_unique());
    assert!(!rc2.is_unique());

    drop(rc2);
    assert_eq!(rc1.strong_count(), 1);
    assert!(rc1.is_unique());
  }

  #[test]
  fn test_try_unwrap() {
    let arena = Arena::new();
    let rc = ArenaRc::new(42, &arena);
    let value = ArenaRc::try_unwrap(rc).unwrap();
    assert_eq!(value, 42);
  }

  #[test]
  fn test_try_unwrap_fails_with_multiple_refs() {
    let arena = Arena::new();
    let rc1 = ArenaRc::new(42, &arena);
    let rc2 = rc1.clone();
    let result = ArenaRc::try_unwrap(rc1);
    assert!(result.is_err());
    let rc1 = result.unwrap_err();
    assert_eq!(*rc1, 42);
    drop(rc2);
  }

  #[test]
  fn test_get_mut() {
    let arena = Arena::new();
    let mut rc = ArenaRc::new(42, &arena);
    {
      let value = rc.get_mut().unwrap();
      *value = 100;
    }
    assert_eq!(*rc, 100);

    let _rc2 = rc.clone();
    assert!(rc.get_mut().is_none());
  }

  #[test]
  fn test_ptr_equality() {
    let arena = Arena::new();
    let rc1 = ArenaRc::new(42, &arena);
    let rc2 = rc1.clone();
    let rc3 = ArenaRc::new(42, &arena);

    assert!(ArenaRc::ptr_eq(&rc1, &rc2));
    assert!(!ArenaRc::ptr_eq(&rc1, &rc3));
  }

  #[test]
  fn test_new_default() {
    let arena = Arena::new();
    let rc = ArenaRc::<i32>::new_default(&arena);
    assert_eq!(*rc, 0);
  }
}
