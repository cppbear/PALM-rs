// Answer 0

#[derive(Debug)]
struct TestStruct;

impl TestStruct {
    fn description(&self) -> &str {
        "TestStruct Description"
    }
}

impl std::fmt::Display for TestStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.description())
    }
}

#[test]
fn test_fmt() {
    let test_struct = TestStruct;
    let mut output = String::new();
    let result = write!(&mut output, "{}", test_struct);
    assert!(result.is_ok());
    assert_eq!(output, "TestStruct Description");
}

