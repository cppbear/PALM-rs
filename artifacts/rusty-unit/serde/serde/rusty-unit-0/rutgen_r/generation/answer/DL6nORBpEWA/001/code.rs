// Answer 0

#[derive(Debug)]
struct TestStruct;

impl std::fmt::Debug for TestStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestStruct")
    }
}

#[test]
fn test_expecting() {
    let mut formatter = std::fmt::Formatter::default();
    let test_struct = TestStruct;

    let result = test_struct.expecting(&mut formatter);
    assert_eq!(result, Ok(()));
}


