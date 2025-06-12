// Answer 0

#[test]
fn test_ident_fragment_fmt() {
    struct TestStruct;
    
    impl IdentFragment for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestStruct")
        }
    }
    
    let instance = TestStruct;
    let mut output = String::new();
    let result = instance.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "TestStruct");
}

#[test]
fn test_ident_fragment_fmt_with_string() {
    let instance = String::from("Hello");
    let mut output = String::new();
    let result = instance.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "Hello");
}

