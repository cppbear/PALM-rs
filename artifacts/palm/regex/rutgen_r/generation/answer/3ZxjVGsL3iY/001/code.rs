// Answer 0

#[derive(Debug, PartialEq)]
struct ByteClassSet([bool; 256]);

impl ByteClassSet {
    fn new() -> Self {
        ByteClassSet([false; 256])
    }
}

#[test]
fn test_byte_class_set_new() {
    let bcs = ByteClassSet::new();
    assert_eq!(bcs, ByteClassSet([false; 256]));
}

#[test]
fn test_byte_class_set_new_length() {
    let bcs = ByteClassSet::new();
    assert_eq!(bcs.0.len(), 256);
}

#[test]
fn test_byte_class_set_new_elements() {
    let bcs = ByteClassSet::new();
    for &value in bcs.0.iter() {
        assert_eq!(value, false);
    }
}

