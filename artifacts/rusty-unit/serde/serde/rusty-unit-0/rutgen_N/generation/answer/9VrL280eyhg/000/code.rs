// Answer 0

#[derive(Debug)]
struct TestStruct;

impl TestStruct {
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("any value")
    }
}

#[test]
fn test_expecting() {
    let test_struct = TestStruct;
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        assert!(test_struct.expecting(&mut formatter).is_ok());
    }
    assert_eq!(output, "any value");
}

