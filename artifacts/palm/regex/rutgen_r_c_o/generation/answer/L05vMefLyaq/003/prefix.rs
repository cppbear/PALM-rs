// Answer 0

#[test]
fn test_unclosed_class_error_empty_stack() {
    let parser = Parser {
        stack_class: RefCell::new(vec![]),
        ..Default::default()
    };
    let parser_i = ParserI::new(parser, "");
    parser_i.unclosed_class_error();
}

#[test]
fn test_unclosed_class_error_one_open_class() {
    let open_class = ClassState::Open {
        union: ast::ClassSetUnion::default(),
        set: ast::ClassBracketed {
            span: Span { start: 0, end: 1 },
            negated: false,
            kind: ast::ClassSet::default(),
        },
    };
    let parser = Parser {
        stack_class: RefCell::new(vec![open_class]),
        ..Default::default()
    };
    let parser_i = ParserI::new(parser, "");
    parser_i.unclosed_class_error();
}

#[test]
fn test_unclosed_class_error_multiple_non_open_classes() {
    let non_open_class1 = ClassState::Op {
        kind: ast::ClassSetBinaryOpKind::default(),
        lhs: ast::ClassSet::default(),
    };
    let non_open_class2 = ClassState::Op {
        kind: ast::ClassSetBinaryOpKind::default(),
        lhs: ast::ClassSet::default(),
    };
    let parser = Parser {
        stack_class: RefCell::new(vec![non_open_class1, non_open_class2]),
        ..Default::default()
    };
    let parser_i = ParserI::new(parser, "");
    parser_i.unclosed_class_error();
}

