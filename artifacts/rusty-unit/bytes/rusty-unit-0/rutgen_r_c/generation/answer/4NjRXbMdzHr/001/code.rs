// Answer 0

#[test]
fn test_slice_ref_empty_subset() {
    let bytes = Bytes::from_static(b"123456");
    let subset: &[u8] = &[];
    let subslice = bytes.slice_ref(subset);
    assert_eq!(subslice.len(), 0);
}

#[test]
#[should_panic(expected = "subset pointer")]
fn test_slice_ref_subset_pointer_smaller() {
    let bytes = Bytes::from_static(b"123456");
    let subset = &b"abcdef"[..]; // Invalid subset, causes panic
    bytes.slice_ref(subset);
}

#[test]
#[should_panic(expected = "subset is out of bounds")]
fn test_slice_ref_subset_out_of_bounds() {
    let bytes = Bytes::from_static(b"123456");
    let subset = &b"4567"[..]; // Valid subset but not contained in bytes
    bytes.slice_ref(subset);
}

#[test]
fn test_slice_ref_valid_subset() {
    let bytes = Bytes::from_static(b"abcdef");
    let subset = &bytes[1..4]; // Valid subset
    let subslice = bytes.slice_ref(subset);
    assert_eq!(&subslice[..], b"BCD");
}

