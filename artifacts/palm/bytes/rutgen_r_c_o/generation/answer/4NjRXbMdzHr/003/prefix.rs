// Answer 0

#[test]
#[should_panic]
fn test_slice_ref_subset_out_of_bounds() {
    let bytes = Bytes::from_static(&[1, 2, 3, 4, 5]);
    let subset = &[1, 2, 3, 4, 5]; // sub_p == bytes_p, sub_len is 6, which is out of bounds.
    let _ = bytes.slice_ref(subset);
}

#[test]
#[should_panic]
fn test_slice_ref_subset_starts_before() {
    let bytes = Bytes::from_static(&[1, 2, 3, 4, 5]);
    let subset = &[2, 3, 4]; // sub_p < bytes_p (here bytes_p is 0)
    let _ = bytes.slice_ref(subset);
}

#[test]
fn test_slice_ref_empty_subset() {
    let bytes = Bytes::from_static(&[1, 2, 3, 4, 5]);
    let subset = &[]; // empty slice is allowed
    let result = bytes.slice_ref(subset);
    assert!(result.is_empty());
}

#[test]
fn test_slice_ref_exact_match() {
    let bytes = Bytes::from_static(&[1, 2, 3, 4, 5]);
    let subset = &[1, 2, 3, 4, 5]; // Exact match, should be valid
    let result = bytes.slice_ref(subset);
    assert_eq!(result.len(), 5);
}

#[test]
fn test_slice_ref_valid_subset_within_bounds() {
    let bytes = Bytes::from_static(&[1, 2, 3, 4, 5]);
    let subset = &[2, 3, 4]; // Valid subset within bounds
    let result = bytes.slice_ref(subset);
    assert_eq!(result.len(), 3);
    assert_eq!(&*result, &[2, 3, 4]);
}

