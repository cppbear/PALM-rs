// Answer 0

#[test]
fn test_unsplit_empty_self() {
    let mut buf = BytesMut::new();
    let other = BytesMut::from_vec(vec![1, 2, 3]);
    buf.unsplit(other);
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_unsplit_non_empty_self_with_contiguous() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    let split = buf.split_off(3); // split_off creates a separate part
    assert_eq!(split.len(), 2); // should have 2 elements

    // Now unsplit should merge them back
    buf.unsplit(split);
    assert_eq!(buf.len(), 5);
    assert_eq!(buf.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_unsplit_non_empty_self_with_non_contiguous() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    let split = BytesMut::from_vec(vec![6, 7, 8]); // this will not be adjacent
    buf.unsplit(split); // will extend from the non-contiguous slice

    assert_eq!(buf.len(), 8);
    assert_eq!(buf.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8]);
}

