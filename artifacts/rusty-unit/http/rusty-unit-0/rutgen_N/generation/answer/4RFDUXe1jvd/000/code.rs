// Answer 0

#[test]
fn test_write_unreachable() {
    struct TestStruct;

    impl TestStruct {
        fn write(&mut self, _: &[u8]) {
            unreachable!("TypeId calls write_u64");
        }
    }

    let mut test_instance = TestStruct;
    let result = std::panic::catch_unwind(|| {
        test_instance.write(b"test");
    });

    assert!(result.is_err());
}

