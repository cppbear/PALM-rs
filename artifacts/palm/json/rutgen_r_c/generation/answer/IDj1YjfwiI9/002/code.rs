// Answer 0

fn test_invalid_type_u64() {
    struct MockExpected;

    impl Expected for MockExpected {
        // Implement necessary trait methods if required for your tests
    }

    let exp = MockExpected;

    let number = ParserNumber::U64(42);
    let result = number.invalid_type(&exp);
    
    if let Err(e) = result { 
        panic!("Expected no error, but got: {:?}", e);
    }
}

fn test_invalid_type_f64() {
    struct MockExpected;

    impl Expected for MockExpected {
        // Implement necessary trait methods if required for your tests
    }

    let exp = MockExpected;

    let number = ParserNumber::F64(3.14);
    let result = number.invalid_type(&exp);
    
    if let Err(e) = result { 
        panic!("Expected no error, but got: {:?}", e);
    }
}

fn test_invalid_type_i64() {
    struct MockExpected;

    impl Expected for MockExpected {
        // Implement necessary trait methods if required for your tests
    }

    let exp = MockExpected;

    let number = ParserNumber::I64(-10);
    let result = number.invalid_type(&exp);
    
    if let Err(e) = result { 
        panic!("Expected no error, but got: {:?}", e);
    }
}

#[cfg(feature = "arbitrary_precision")]
fn test_invalid_type_string() {
    struct MockExpected;

    impl Expected for MockExpected {
        // Implement necessary trait methods if required for your tests
    }

    let exp = MockExpected;

    let number = ParserNumber::String("123456".to_string());
    let result = number.invalid_type(&exp);
    
    if let Err(e) = result { 
        panic!("Expected no error, but got: {:?}", e);
    }
}

