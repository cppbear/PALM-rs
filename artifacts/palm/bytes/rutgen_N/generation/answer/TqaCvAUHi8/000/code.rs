// Answer 0

#[test]
fn test_chunk() {
    struct TestBytes {
        data: Vec<u8>,
    }

    impl TestBytes {
        fn new(data: Vec<u8>) -> Self {
            TestBytes { data }
        }

        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn chunk(&self) -> &[u8] {
            self.as_slice()
        }
    }

    let test_instance = TestBytes::new(vec![1, 2, 3, 4, 5]);
    let result = test_instance.chunk();
    assert_eq!(result, &[1, 2, 3, 4, 5]);

    let empty_instance = TestBytes::new(vec![]);
    let empty_result = empty_instance.chunk();
    assert_eq!(empty_result, &[]);
}

