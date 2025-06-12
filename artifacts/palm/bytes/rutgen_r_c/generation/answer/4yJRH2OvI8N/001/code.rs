// Answer 0

#[test]
fn test_first_ref_with_integer_chain() {
    struct IntegerBuf {
        value: i32,
    }
    
    let chain = Chain::new(IntegerBuf { value: 42 }, IntegerBuf { value: 100 });
    
    assert_eq!(chain.first_ref().value, 42);
}

#[test]
fn test_first_ref_with_string_chain() {
    struct StringBuf {
        value: String,
    }
    
    let chain = Chain::new(StringBuf { value: String::from("hello") }, StringBuf { value: String::from("world") });
    
    assert_eq!(chain.first_ref().value, "hello");
}

#[test]
fn test_first_ref_with_empty_chain() {
    struct EmptyBuf {
        value: String,
    }
    
    let chain = Chain::new(EmptyBuf { value: String::new() }, EmptyBuf { value: String::from("not_empty") });
    
    assert_eq!(chain.first_ref().value, "");
}

#[test]
fn test_first_ref_with_large_integer_chain() {
    struct LargeIntegerBuf {
        value: i64,
    }
    
    let chain = Chain::new(LargeIntegerBuf { value: 123456789012345 }, LargeIntegerBuf { value: 678910111213141 });
    
    assert_eq!(chain.first_ref().value, 123456789012345);
}

