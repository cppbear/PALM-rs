// Answer 0

#[test]
fn test_len() {
    struct TestStruct;

    impl TestStruct {
        pub fn len(&self) -> usize {
            0
        }
    }

    let instance = TestStruct;
    assert_eq!(instance.len(), 0);
}

#[test]
fn test_len_empty_instance() {
    struct EmptyStruct;

    impl EmptyStruct {
        pub fn len(&self) -> usize {
            0
        }
    }

    let instance = EmptyStruct;
    assert_eq!(instance.len(), 0);
}

