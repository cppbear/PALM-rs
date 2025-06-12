// Answer 0

#[test]
fn test_next_after_empty() {
    struct TestStruct;

    impl TestStruct {
        fn next_after_empty(&self, _text: &[u8], i: usize) -> usize {
            i + 1
        }
    }

    let test_instance = TestStruct;
    let result = test_instance.next_after_empty(b"", 0);
    assert_eq!(result, 1);
}

#[test]
fn test_next_after_empty_increments() {
    struct TestStruct;

    impl TestStruct {
        fn next_after_empty(&self, _text: &[u8], i: usize) -> usize {
            i + 1
        }
    }

    let test_instance = TestStruct;
    let result = test_instance.next_after_empty(b"", 5);
    assert_eq!(result, 6);
}

