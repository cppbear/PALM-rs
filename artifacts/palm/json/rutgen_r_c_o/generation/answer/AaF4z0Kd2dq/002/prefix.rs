// Answer 0

#[test]
fn test_next_empty_slice() {
    let mut reader = SliceRead { slice: &[], index: 0 };
    let result = reader.next();
}

#[test]
fn test_next_at_end_of_non_empty_slice() {
    let data = vec![1u8, 2, 3];
    let mut reader = SliceRead { slice: &data, index: 3 };
    let result = reader.next();
}

#[test]
fn test_next_after_reading_from_non_empty_slice() {
    let data = vec![4u8, 5, 6];
    let mut reader = SliceRead { slice: &data, index: 2 };
    let result = reader.next();
}

#[test]
fn test_next_single_element_slice_at_end() {
    let data = vec![7u8];
    let mut reader = SliceRead { slice: &data, index: 1 };
    let result = reader.next();
}

#[test]
fn test_next_large_slice_at_end() {
    let data: Vec<u8> = (0u8..=255).collect();
    let mut reader = SliceRead { slice: &data, index: 256 };
    let result = reader.next();
}

