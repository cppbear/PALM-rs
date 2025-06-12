// Answer 0

#[derive(Debug)]
struct TestError;

struct TestStruct;

impl TestStruct {
    fn unit_variant(self) -> Result<(), TestError> {
        Ok(())
    }
}

#[test]
fn test_unit_variant_success() {
    let test_instance = TestStruct;
    let result = test_instance.unit_variant();
    assert!(result.is_ok());
}

