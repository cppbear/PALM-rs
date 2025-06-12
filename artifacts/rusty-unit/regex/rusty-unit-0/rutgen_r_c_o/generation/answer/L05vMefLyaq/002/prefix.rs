// Answer 0

#[test]
fn test_unclosed_class_error_with_one_open_class() {
    let span = Span { start: 0, end: 5 };
    let set = ClassBracketed { span, negated: false, kind: ast::ClassSet::Union(vec![]) };
    let state = ClassState::Open { union: ast::ClassSetUnion::default(), set };
    
    let parser = Parser {
        stack_class: RefCell::new(vec![state]),
        ..Default::default()
    };
    
    let parser_instance = ParserI { parser: &parser, pattern: "[a-z]" };
    
    let _ = parser_instance.unclosed_class_error();
}

#[test]
fn test_unclosed_class_error_with_multiple_open_classes() {
    let span1 = Span { start: 0, end: 5 };
    let set1 = ClassBracketed { span: span1, negated: false, kind: ast::ClassSet::Union(vec![]) };
    let state1 = ClassState::Open { union: ast::ClassSetUnion::default(), set: set1 };
    
    let span2 = Span { start: 6, end: 10 };
    let set2 = ClassBracketed { span: span2, negated: false, kind: ast::ClassSet::Union(vec![]) };
    let state2 = ClassState::Open { union: ast::ClassSetUnion::default(), set: set2 };
    
    let parser = Parser {
        stack_class: RefCell::new(vec![state1, state2]),
        ..Default::default()
    };
    
    let parser_instance = ParserI { parser: &parser, pattern: "[a-z][0-9]" };
    
    let _ = parser_instance.unclosed_class_error();
}

#[test]
fn test_unclosed_class_error_with_nested_open_classes() {
    let span1 = Span { start: 0, end: 5 };
    let set1 = ClassBracketed { span: span1, negated: true, kind: ast::ClassSet::Union(vec![]) };
    let state1 = ClassState::Open { union: ast::ClassSetUnion::default(), set: set1 };

    let span2 = Span { start: 6, end: 10 };
    let set2 = ClassBracketed { span: span2, negated: false, kind: ast::ClassSet::Union(vec![]) };
    let state2 = ClassState::Open { union: ast::ClassSetUnion::default(), set: set2 };

    let parser = Parser {
        stack_class: RefCell::new(vec![state1, state2]),
        ..Default::default()
    };

    let parser_instance = ParserI { parser: &parser, pattern: "[^a-z][0-9]" };

    let _ = parser_instance.unclosed_class_error();
}

#[test]
#[should_panic(expected = "no open character class found")]
fn test_unclosed_class_error_no_open_classes() {
    let parser = Parser {
        stack_class: RefCell::new(vec![]),
        ..Default::default()
    };

    let parser_instance = ParserI { parser: &parser, pattern: "" };

    let _ = parser_instance.unclosed_class_error();
}

