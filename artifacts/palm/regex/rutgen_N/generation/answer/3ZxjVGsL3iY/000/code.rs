// Answer 0

#[derive(Debug)]
struct ByteClassSet([bool; 256]);

impl ByteClassSet {
    fn new() -> Self {
        ByteClassSet([false; 256])
    }
}

#[test]
fn test_byte_class_set_new() {
    let byte_class_set = ByteClassSet::new();
    assert_eq!(byte_class_set.0, [false; 256]);
}

