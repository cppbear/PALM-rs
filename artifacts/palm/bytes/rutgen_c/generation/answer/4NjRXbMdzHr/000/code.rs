// Answer 0

#[test]
fn test_slice_ref_empty_subset() {
    let bytes = Bytes::from_static(b"012345678");
    let subset = &b""[..];
    let subslice = bytes.slice_ref(subset);
    assert_eq!(subslice.len(), 0);
}

#[test]
fn test_slice_ref_valid_subset() {
    let bytes = Bytes::from_static(b"012345678");
    let subset = &bytes[2..6]; // Corresponds to "2345"
    let subslice = bytes.slice_ref(subset);
    assert_eq!(&*subslice, b"2345");
}

#[test]
#[should_panic(expected = "subset pointer")]
fn test_slice_ref_subset_pointer_too_small() {
    let bytes = Bytes::from_static(b"012345678");
    let subset = &b"01"[..]; // Pointer is less than bytes pointer
    bytes.slice_ref(subset);
}

#[test]
#[should_panic(expected = "subset is out of bounds")]
fn test_slice_ref_subset_out_of_bounds() {
    let bytes = Bytes::from_static(b"012345678");
    let subset = &b"6789"[..]; // Pointer is out of bounds
    bytes.slice_ref(subset);
}

#[test]
fn test_slice_ref_full_subset() {
    let bytes = Bytes::from_static(b"012345678");
    let subset = &bytes[..]; // Full bytes
    let subslice = bytes.slice_ref(subset);
    assert_eq!(&*subslice, b"012345678");
}

