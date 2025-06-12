// Answer 0

#[test]
fn test_unit_variant_success() {
    struct TestStruct;

    impl TestStruct {
        fn unit_variant(self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    let test_instance = TestStruct;
    let result = test_instance.unit_variant();
    assert_eq!(result, Ok(()));
}

