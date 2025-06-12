// Answer 0

#[test]
fn test_byte_offset_with_some() {
    struct MockIterator {
        byte_offset_val: usize,
    }

    impl MockIterator {
        fn byte_offset(&self) -> usize {
            self.byte_offset_val
        }
    }

    struct TestStruct {
        ch: Option<char>,
        iter: MockIterator,
    }

    let test_instance = TestStruct {
        ch: Some('a'),
        iter: MockIterator { byte_offset_val: 10 },
    };

    let result = test_instance.byte_offset();
    assert_eq!(result, 9);
}

