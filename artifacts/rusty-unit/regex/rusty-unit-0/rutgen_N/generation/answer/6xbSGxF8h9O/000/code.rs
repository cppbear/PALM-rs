// Answer 0

#[test]
fn test_is_empty_when_length_zero() {
    struct TestStruct {
        len: usize,
    }

    impl TestStruct {
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let test_instance = TestStruct { len: 0 };
    assert!(test_instance.is_empty());
}

#[test]
fn test_is_empty_when_length_non_zero() {
    struct TestStruct {
        len: usize,
    }

    impl TestStruct {
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let test_instance = TestStruct { len: 1 };
    assert!(!test_instance.is_empty());
}

