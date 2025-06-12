// Answer 0

#[test]
fn test_buckets() {
    struct TestStruct {
        bucket_mask: usize,
    }

    impl TestStruct {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }
    }

    let test_instance = TestStruct { bucket_mask: 4 };
    assert_eq!(test_instance.buckets(), 5);

    let test_instance_zero = TestStruct { bucket_mask: 0 };
    assert_eq!(test_instance_zero.buckets(), 1);

    let test_instance_large = TestStruct { bucket_mask: usize::MAX - 1 };
    assert_eq!(test_instance_large.buckets(), usize::MAX);
}

