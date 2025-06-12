// Answer 0

#[test]
fn test_visit_bytes_valid_utf8() {
    let mut result_string = String::new();
    let visitor = StringInPlaceVisitor(&mut result_string);
    let input_bytes = b"Hello, world!";
    
    let result: Result<(), _> = visitor.visit_bytes(input_bytes);
    
    assert!(result.is_ok());
    assert_eq!(result_string, "Hello, world!");
}

#[test]
fn test_visit_bytes_invalid_utf8() {
    let mut result_string = String::new();
    let visitor = StringInPlaceVisitor(&mut result_string);
    let input_bytes = &[0, 159, 146, 150]; // Invalid UTF-8 sequence
    
    let result: Result<(), _> = visitor.visit_bytes(input_bytes);
    
    assert!(result.is_err());
}

#[test]
fn test_visit_bytes_empty() {
    let mut result_string = String::new();
    let visitor = StringInPlaceVisitor(&mut result_string);
    let input_bytes: &[u8] = &[];
    
    let result: Result<(), _> = visitor.visit_bytes(input_bytes);
    
    assert!(result.is_ok());
    assert_eq!(result_string, "");
}

