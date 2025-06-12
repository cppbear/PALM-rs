// Answer 0

#[test]
fn test_split_off_middle() {
    let mut a = Bytes::copy_from_slice(b"hello world");
    let b = a.split_off(5);

    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b" world");
}

#[test]
fn test_split_off_full() {
    let mut a = Bytes::copy_from_slice(b"some data");
    let b = a.split_off(9); // at equals length

    assert_eq!(a.len(), 9);
    assert_eq!(b.len(), 0);
}

#[test]
fn test_split_off_empty() {
    let mut a = Bytes::copy_from_slice(b"data");
    let b = a.split_off(0); // at equals 0

    assert_eq!(a.len(), 0);
    assert_eq!(&b[..], b"data");
}

#[test]
#[should_panic(expected = "split_off out of bounds: 10 <= 4")]
fn test_split_off_out_of_bounds_high() {
    let mut a = Bytes::copy_from_slice(b"test");
    let _ = a.split_off(10); // at greater than length
}

#[test]
#[should_panic(expected = "split_off out of bounds: 5 <= 4")]
fn test_split_off_out_of_bounds() {
    let mut a = Bytes::copy_from_slice(b"abcd");
    let _ = a.split_off(5); // at greater than length
}

