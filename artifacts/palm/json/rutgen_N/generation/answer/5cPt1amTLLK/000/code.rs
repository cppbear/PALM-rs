// Answer 0

#[test]
fn test_unit_variant_success() {
    struct TestStruct;

    impl TestStruct {
        fn unit_variant(self) -> Result<(), ()> {
            Ok(())
        }
    }

    let instance = TestStruct;
    let result = instance.unit_variant();
    assert!(result.is_ok());
}

