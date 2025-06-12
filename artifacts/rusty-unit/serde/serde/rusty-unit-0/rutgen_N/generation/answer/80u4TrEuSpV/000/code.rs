// Answer 0

#[derive(Debug)]
struct TestStruct {
    expecting: &'static str,
}

impl TestStruct {
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(self.expecting)
    }
}

#[test]
fn test_expectation() {
    let test_instance = TestStruct { expecting: "expected value" };
    let mut formatter = std::fmt::Formatter::new();
    let result = test_instance.expecting(&mut formatter);
    assert!(result.is_ok());
    let output = formatter.inner().clone();
    assert_eq!(output, "expected value");
}

