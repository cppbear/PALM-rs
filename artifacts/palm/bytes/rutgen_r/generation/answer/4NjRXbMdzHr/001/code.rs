// Answer 0

#[test]
fn test_slice_ref_subset_empty() {
    use bytes::Bytes;

    let bytes = Bytes::from(&b"012345678"[..]);
    let subset: &[u8] = &[];

    let subslice = bytes.slice_ref(subset);
    assert_eq!(&subslice[..], b"");
}

#[test]
#[should_panic]
fn test_slice_ref_subset_not_contained() {
    use bytes::Bytes;

    let bytes = Bytes::from(&b"012345678"[..]);
    let subset = &b"123"[..]; // This subset is not a slice of the `Bytes`

    // This should panic
    let _ = bytes.slice_ref(subset);
}

#[test]
fn test_slice_ref_valid_subset() {
    use bytes::Bytes;

    let bytes = Bytes::from(&b"012345678"[..]);
    let subset = &bytes[2..6];

    let subslice = bytes.slice_ref(subset);
    assert_eq!(&subslice[..], b"2345");
}

