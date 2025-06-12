// Answer 0

#[test]
fn test_error_creates_correct_error() {
    let span = Span { start: 0, end: 1 };
    let kind = ast::ErrorKind::CaptureLimitExceeded;
    let pattern = "abc";
    let parser_instance = ParserI::new(Parser { 
        pos: Cell::new(0), 
        capture_index: Cell::new(0), 
        nest_limit: 10, 
        octal: false, 
        initial_ignore_whitespace: false, 
        ignore_whitespace: Cell::new(false), 
        comments: RefCell::new(vec![]), 
        stack_group: RefCell::new(vec![]), 
        stack_class: RefCell::new(vec![]), 
        capture_names: RefCell::new(vec![]), 
        scratch: RefCell::new(String::new()), 
    }, pattern);
    
    let error = parser_instance.error(span, kind);
    
    assert_eq!(error.kind, kind);
    assert_eq!(error.pattern, pattern);
    assert_eq!(error.span, span);
} 

#[test]
fn test_error_creates_correct_error_with_different_kind() {
    let span = Span { start: 2, end: 3 };
    let kind = ast::ErrorKind::ClassRangeInvalid;
    let pattern = "xyz";
    let parser_instance = ParserI::new(Parser { 
        pos: Cell::new(0), 
        capture_index: Cell::new(0), 
        nest_limit: 10, 
        octal: false, 
        initial_ignore_whitespace: false, 
        ignore_whitespace: Cell::new(false), 
        comments: RefCell::new(vec![]), 
        stack_group: RefCell::new(vec![]), 
        stack_class: RefCell::new(vec![]), 
        capture_names: RefCell::new(vec![]), 
        scratch: RefCell::new(String::new()), 
    }, pattern);
    
    let error = parser_instance.error(span, kind);
    
    assert_eq!(error.kind, kind);
    assert_eq!(error.pattern, pattern);
    assert_eq!(error.span, span);
}

