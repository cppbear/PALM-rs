// Answer 0

#[test]
fn test_slice_ref_non_empty() {
    use bytes::Bytes;

    let bytes = Bytes::from(&b"012345678"[..]);
    let as_slice = bytes.as_ref();
    let subset = &as_slice[0..9]; // This slice is the entire range, satisfying the bounds.
    let subslice = bytes.slice_ref(&subset);
    assert_eq!(&subslice[..], b"012345678"); // Expecting the entire original Bytes back.
}

#[test]
fn test_slice_ref_empty_subslice() {
    use bytes::Bytes;

    let bytes = Bytes::from(&b"012345678"[..]);
    let subset = &[] as &[u8]; // This slice is empty, which is a valid input.
    let subslice = bytes.slice_ref(&subset);
    assert_eq!(&subslice[..], b""); // Expecting an empty Bytes back.
}

#[test]
#[should_panic(expected = "subset pointer")]
fn test_slice_ref_subset_pointer_smaller_than_self() {
    use bytes::Bytes;

    let bytes = Bytes::from(&b"012345678"[..]);
    let invalid_subset = &b"2345"[..]; // Subset that's not derived from the original bytes.
    let _ = bytes.slice_ref(invalid_subset);
}

#[test]
#[should_panic]
fn test_slice_ref_out_of_bounds() {
    use bytes::Bytes;

    let bytes = Bytes::from(&b"012345678"[..]);
    let out_of_bounds_subset = &b"56789"[..]; // This subset reference exceeds the length of the original.
    let _ = bytes.slice_ref(out_of_bounds_subset);
}

