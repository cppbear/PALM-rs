// Answer 0

#[test]
fn test_new_function_returns_none() {
    struct Literals;
    
    let literals = Literals;
    
    let result = regex::new(&literals);
    
    assert_eq!(result, None);
}

