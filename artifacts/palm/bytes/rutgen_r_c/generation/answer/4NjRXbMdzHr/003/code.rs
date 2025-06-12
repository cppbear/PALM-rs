// Answer 0

#[test]
fn test_slice_ref_empty_subset() {
    let bytes = Bytes::from_static(b"example");
    let subset = &[]; // empty subset
    let subslice = bytes.slice_ref(subset);
    assert_eq!(subslice.len(), 0);
}

#[test]
#[should_panic(expected = "subset pointer")]
fn test_slice_ref_subset_pointer_smaller() {
    let bytes = Bytes::from_static(b"example");
    let subset = &b"ample"[..]; // subset pointer is smaller
    let _ = bytes.slice_ref(subset);
}

#[test]
#[should_panic(expected = "subset is out of bounds")]
fn test_slice_ref_out_of_bounds() {
    let bytes = Bytes::from_static(b"example");
    let subset = &b"exampleXXX"[..]; // subset out of bounds
    let _ = bytes.slice_ref(subset);
}

#[test]
fn test_slice_ref_valid_subset() {
    let bytes = Bytes::from_static(b"example");
    let subset = &b"exam"[..]; // valid subset
    let subslice = bytes.slice_ref(subset);
    assert_eq!(&subslice[..], b"exam");
}

#[test]
#[should_panic(expected = "subset is out of bounds")]
fn test_slice_ref_invalid_subset_bounds() {
    let bytes = Bytes::from_static(b"example");
    let subset = &b"amplee"[..]; // overflow beyond bounds
    let _ = bytes.slice_ref(subset);
}

