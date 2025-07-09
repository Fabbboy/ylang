use std::{
  borrow::Borrow,
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
  sync::atomic::{
    AtomicUsize,
    Ordering as AtomicOrdering,
  },
};

use crate::arena::Arena;

struct ArenaArcInner<T> {
  strong: AtomicUsize,
  data: T,
}

/// An atomically reference-counted pointer backed by an arena.
///
/// This is similar to `std::sync::Arc` but allocates the inner data
/// in the provided arena instead of on the heap.
pub struct ArenaArc<'a, T> {
  ptr: NonNull<ArenaArcInner<T>>,
  _arena: &'a Arena,
}

impl<'a, T> ArenaArc<'a, T> {
  /// Constructs a new `ArenaArc<T>` in the given arena.
  pub fn new(data: T, arena: &'a Arena) -> Self {
    let inner = arena.alloc(ArenaArcInner {
      strong: AtomicUsize::new(1),
      data,
    });

    Self {
      ptr: NonNull::from(inner),
      _arena: arena,
    }
  }

  /// Returns the number of strong references to this allocation.
  pub fn strong_count(&self) -> usize {
    unsafe { self.ptr.as_ref().strong.load(AtomicOrdering::SeqCst) }
  }

  /// Returns `true` if there are no other `ArenaArc` pointers to this allocation.
  pub fn is_unique(&self) -> bool {
    self.strong_count() == 1
  }

  /// Gets a mutable reference to the inner value if this `ArenaArc` is unique.
  pub fn get_mut(&mut self) -> Option<&mut T> {
    if self.is_unique() {
      // SAFETY: We have exclusive access to the ArenaArc and there's only one reference
      unsafe { Some(&mut self.ptr.as_mut().data) }
    } else {
      None
    }
  }

  /// Returns a raw pointer to the inner data.
  pub fn as_ptr(&self) -> *const T {
    unsafe { &self.ptr.as_ref().data as *const T }
  }

  /// Creates a new `ArenaArc` from a raw pointer.
  ///
  /// # Safety
  /// The raw pointer must have been returned by `into_raw` and the arena
  /// must still be valid.
  pub unsafe fn from_raw(ptr: *const T, arena: &'a Arena) -> Self {
    // Calculate the offset to the ArenaArcInner from the data field
    unsafe {
      let dummy = std::ptr::null::<ArenaArcInner<T>>();
      let data_offset = std::ptr::addr_of!((*dummy).data) as usize;
      let inner_ptr = (ptr as *const u8).sub(data_offset) as *mut ArenaArcInner<T>;

      Self {
        ptr: NonNull::new_unchecked(inner_ptr),
        _arena: arena,
      }
    }
  }

  /// Consumes the `ArenaArc`, returning the wrapped pointer.
  pub fn into_raw(this: Self) -> *const T {
    let ptr = this.as_ptr();
    std::mem::forget(this);
    ptr
  }

  /// Provides a raw pointer to the data.
  ///
  /// This is equivalent to `ArenaArc::as_ptr`, but can be called on temporaries.
  pub fn ptr_eq(this: &Self, other: &Self) -> bool {
    this.ptr == other.ptr
  }

  /// Attempts to downgrade this `ArenaArc` to a weak reference.
  /// This would require implementing weak references, which is not done here.
  /// This method is provided for API compatibility but always returns None.
  pub fn downgrade(_this: &Self) -> Option<()> {
    // Weak references not implemented for arena-based allocation
    None
  }

  /// Returns the inner value, if this `ArenaArc` has exactly one reference.
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

impl<'a, T: Default> ArenaArc<'a, T> {
  /// Creates a new `ArenaArc<T>` with the default value of `T` in the given arena.
  pub fn new_default(arena: &'a Arena) -> Self {
    Self::new(T::default(), arena)
  }
}

// Utility functions for ArenaArc

impl<'a, T> Clone for ArenaArc<'a, T> {
  fn clone(&self) -> Self {
    let old_size = unsafe {
      self
        .ptr
        .as_ref()
        .strong
        .fetch_add(1, AtomicOrdering::SeqCst)
    };

    // Check for overflow
    if old_size > (isize::MAX as usize) / 2 {
      panic!("ArenaArc reference count overflow");
    }

    Self {
      ptr: self.ptr,
      _arena: self._arena,
    }
  }
}

impl<'a, T> Deref for ArenaArc<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &self.ptr.as_ref().data }
  }
}

impl<'a, T> Drop for ArenaArc<'a, T> {
  fn drop(&mut self) {
    let old_count = unsafe {
      self
        .ptr
        .as_ref()
        .strong
        .fetch_sub(1, AtomicOrdering::SeqCst)
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

impl<'a, T> AsRef<T> for ArenaArc<'a, T> {
  fn as_ref(&self) -> &T {
    &**self
  }
}

impl<'a, T> Borrow<T> for ArenaArc<'a, T> {
  fn borrow(&self) -> &T {
    &**self
  }
}

impl<'a, T: Debug> Debug for ArenaArc<'a, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Debug::fmt(&**self, f)
  }
}

impl<'a, T: Display> Display for ArenaArc<'a, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Display::fmt(&**self, f)
  }
}

impl<'a, T: PartialEq> PartialEq for ArenaArc<'a, T> {
  fn eq(&self, other: &Self) -> bool {
    **self == **other
  }
}

impl<'a, T: Eq> Eq for ArenaArc<'a, T> {}

impl<'a, T: PartialOrd> PartialOrd for ArenaArc<'a, T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    (**self).partial_cmp(&**other)
  }
}

impl<'a, T: Ord> Ord for ArenaArc<'a, T> {
  fn cmp(&self, other: &Self) -> Ordering {
    (**self).cmp(&**other)
  }
}

impl<'a, T: Hash> Hash for ArenaArc<'a, T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    (**self).hash(state)
  }
}

// Send and Sync implementations - same as std::sync::Arc
unsafe impl<'a, T: Sync + Send> Send for ArenaArc<'a, T> {}
unsafe impl<'a, T: Sync + Send> Sync for ArenaArc<'a, T> {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_functionality() {
    let arena = Arena::new();
    let arc = ArenaArc::new(42, &arena);
    assert_eq!(*arc, 42);
    assert_eq!(arc.strong_count(), 1);
    assert!(arc.is_unique());
  }

  #[test]
  fn test_clone_and_reference_counting() {
    let arena = Arena::new();
    let arc1 = ArenaArc::new(String::from("hello"), &arena);
    assert_eq!(arc1.strong_count(), 1);

    let arc2 = arc1.clone();
    assert_eq!(arc1.strong_count(), 2);
    assert_eq!(arc2.strong_count(), 2);
    assert_eq!(*arc1, *arc2);
    assert!(!arc1.is_unique());
    assert!(!arc2.is_unique());

    drop(arc2);
    assert_eq!(arc1.strong_count(), 1);
    assert!(arc1.is_unique());
  }

  #[test]
  fn test_try_unwrap() {
    let arena = Arena::new();
    let arc = ArenaArc::new(42, &arena);
    let value = ArenaArc::try_unwrap(arc).unwrap();
    assert_eq!(value, 42);
  }

  #[test]
  fn test_try_unwrap_fails_with_multiple_refs() {
    let arena = Arena::new();
    let arc1 = ArenaArc::new(42, &arena);
    let arc2 = arc1.clone();
    let result = ArenaArc::try_unwrap(arc1);
    assert!(result.is_err());
    let arc1 = result.unwrap_err();
    assert_eq!(*arc1, 42);
    drop(arc2);
  }

  #[test]
  fn test_get_mut() {
    let arena = Arena::new();
    let mut arc = ArenaArc::new(42, &arena);
    {
      let value = arc.get_mut().unwrap();
      *value = 100;
    }
    assert_eq!(*arc, 100);

    let _arc2 = arc.clone();
    assert!(arc.get_mut().is_none());
  }

  #[test]
  fn test_ptr_equality() {
    let arena = Arena::new();
    let arc1 = ArenaArc::new(42, &arena);
    let arc2 = arc1.clone();
    let arc3 = ArenaArc::new(42, &arena);

    assert!(ArenaArc::ptr_eq(&arc1, &arc2));
    assert!(!ArenaArc::ptr_eq(&arc1, &arc3));
  }
}
