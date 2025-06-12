// Answer 0

#[test]
fn test_from_name_word() {
    // Given that the name matches "word"
    let name = "word";
    
    // When calling from_name
    let result = regex_syntax::from_name(name);
    
    // Then it should return Some(ClassAsciiKind::Word)
    assert_eq!(result, Some(regex_syntax::ClassAsciiKind::Word));
}

#[test]
fn test_from_name_nonexistent() {
    // Given names that do not match any variants
    let names = [
        "alnum", "alpha", "ascii", "blank", "cntrl", 
        "digit", "graph", "lower", "print", "punct", 
        "space", "upper", "xdigit"
    ];
    
    // When calling from_name for each name
    for &name in &names {
        let result = regex_syntax::from_name(name);
        
        // Then it should return None
        assert_eq!(result, None);
    }
}

