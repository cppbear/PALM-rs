// Answer 0

#[test]
fn test_unit_variant_success() {
    struct TestStruct;

    impl TestStruct {
        fn unit_variant(self) -> Result<(), String> {
            Ok(())
        }
    }

    let instance = TestStruct;
    let result = instance.unit_variant();
    assert_eq!(result, Ok(()));
}

