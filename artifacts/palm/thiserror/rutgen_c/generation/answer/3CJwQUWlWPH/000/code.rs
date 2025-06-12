// Answer 0

#[test]
fn test_fmt() {
    use core::fmt::Formatter;
    use core::fmt::Result;

    struct TestPointer;
    
    impl Pointer for TestPointer {
        fn fmt(&self, formatter: &mut Formatter) -> Result {
            formatter.write_str("TestPointer")
        }
    }
    
    let test_pointer = TestPointer;
    let var = Var(&test_pointer);
    
    let mut formatter = Formatter::default(); // Assuming we have a default constructor
    let result = var.fmt(&mut formatter);
    
    assert!(result.is_ok());
}

#[test]
fn test_fmt_with_different_pointer() {
    use core::fmt::Formatter;
    use core::fmt::Result;

    struct AnotherPointer;
    
    impl Pointer for AnotherPointer {
        fn fmt(&self, formatter: &mut Formatter) -> Result {
            formatter.write_str("AnotherPointer")
        }
    }
    
    let another_pointer = AnotherPointer;
    let var = Var(&another_pointer);
    
    let mut formatter = Formatter::default(); // Assuming we have a default constructor
    let result = var.fmt(&mut formatter);
    
    assert!(result.is_ok());
}

