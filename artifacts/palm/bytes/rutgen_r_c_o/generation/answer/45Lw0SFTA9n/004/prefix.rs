// Answer 0

#[test]
fn test_split_to_empty() {
    let mut a = Bytes::new();
    let b = a.split_to(0);
}

#[test]
fn test_split_to_single_byte() {
    let mut a = Bytes::from_static(b"example");
    let b = a.split_to(0);
}

#[test]
fn test_split_to_full_length() {
    let mut a = Bytes::from_static(b"example");
    let b = a.split_to(7);
}

#[test]
#[should_panic]
fn test_split_to_out_of_bounds() {
    let mut a = Bytes::from_static(b"example");
    let _ = a.split_to(8);
}

#[test]
fn test_split_to_non_empty() {
    let mut a = Bytes::from_static(b"example");
    let b = a.split_to(3);
}

