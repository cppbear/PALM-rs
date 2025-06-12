// Answer 0

#[test]
fn test_empty_class_unicode() {
    struct ClassUnicode {
        ranges: Vec<u8>,
    }

    impl ClassUnicode {
        pub fn new(ranges: Vec<u8>) -> Self {
            Self { ranges }
        }
    }

    let class = ClassUnicode::new(vec![]);
    assert_eq!(class.ranges.len(), 0);
}

