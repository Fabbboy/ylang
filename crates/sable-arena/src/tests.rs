use crate::arena::RawArena;

#[test]
fn test_basic_allocation() {
  let arena = RawArena::with_chunk_size(1024);

  let ref1 = arena.alloc(42).unwrap();
  let ref2 = arena.alloc(24).unwrap();

  assert_eq!(*ref1, 42);
  assert_eq!(*ref2, 24);

  *ref1 = 100;
  assert_eq!(*ref1, 100);
}

#[test]
fn test_slice_allocation_copy() {
  let arena = RawArena::with_chunk_size(1024);

  let values = [1, 2, 3, 4, 5];
  let slice_ref = arena.alloc_slice_copy(&values).unwrap();

  assert_eq!(slice_ref, &mut [1, 2, 3, 4, 5]);
  slice_ref[0] = 10;
  assert_eq!(slice_ref[0], 10);
}

#[test]
fn test_slice_allocation_with_closure() {
  let arena = RawArena::with_chunk_size(1024);

  let slice_ref = arena.alloc_slice_with(5, |i| i * 2).unwrap();

  assert_eq!(slice_ref, &mut [0, 2, 4, 6, 8]);
  slice_ref[0] = 10;
  assert_eq!(slice_ref[0], 10);
}

#[test]
fn test_string_allocation() {
  let arena = RawArena::with_chunk_size(1024);

  let text = arena.alloc_str("Hello, world!").unwrap();
  assert_eq!(text, "Hello, world!");

  let empty = arena.alloc_str("").unwrap();
  assert_eq!(empty, "");
}

#[test]
fn test_soft_cleanup() {
  let arena = RawArena::with_chunk_size(1024);

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
  let arena = RawArena::with_chunk_size(1024);

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
  let arena = RawArena::with_chunk_size(1024);

  #[derive(Debug, PartialEq)]
  struct ZeroSized;

  let ref1 = arena.alloc(ZeroSized).unwrap();
  let ref2 = arena.alloc(ZeroSized).unwrap();

  assert_eq!(*ref1, ZeroSized);
  assert_eq!(*ref2, ZeroSized);
}

#[test]
fn test_large_allocation() {
  let arena = RawArena::with_chunk_size(64); // Small chunks

  let large_array = [1u8; 128]; // Larger than chunk size
  let slice_ref = arena.alloc_slice_copy(&large_array).unwrap();

  assert_eq!(slice_ref.len(), 128);
  assert_eq!(slice_ref[0], 1);
}

#[test]
fn test_clear() {
  let arena = RawArena::with_chunk_size(1024);

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
  let arena = RawArena::with_chunk_size(1024);

  // Test with non-Copy types
  let strings = arena
    .alloc_slice_with(3, |i| format!("item {}", i))
    .unwrap();
  assert_eq!(strings[0], "item 0");
  assert_eq!(strings[1], "item 1");
  assert_eq!(strings[2], "item 2");
}
