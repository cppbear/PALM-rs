// Answer 0

#[test]
fn test_visit_post_with_group() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };

    let span = Span { start: 0, end: 5 };
    let literal = ast::Literal { span, kind: ast::LiteralKind::Char('a'), c: 'a' };
    let group = ast::Group {
        span,
        kind: ast::GroupKind::NonCapturing,
        ast: Box::new(ast::Ast::Literal(literal.clone())),
    };

    translator.flags.set(Flags {
        case_insensitive: Some(true),
        multi_line: Some(false),
        dot_matches_new_line: Some(false),
        swap_greed: Some(false),
        unicode: Some(true),
    });

    let group_ast = ast::Ast::Group(group);
    translator.visit_post(&group_ast).unwrap();
}

#[test]
fn test_visit_post_with_group_and_flags() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };

    let span = Span { start: 0, end: 5 };
    let flag_set = ast::Flags { span, flags: Flags::from_ast(&ast::Flags::default()) };
    let group = ast::Group {
        span,
        kind: ast::GroupKind::CaptureIndex(1),
        ast: Box::new(ast::Ast::Flags(flag_set)),
    };

    translator.flags.set(Flags {
        case_insensitive: Some(true),
        multi_line: Some(true),
        dot_matches_new_line: Some(false),
        swap_greed: Some(false),
        unicode: Some(true),
    });

    let group_ast = ast::Ast::Group(group);
    translator.visit_post(&group_ast).unwrap();
}

#[test]
fn test_visit_post_with_group_repetition() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };

    let span = Span { start: 0, end: 5 };
    let literal = ast::Literal { span, kind: ast::LiteralKind::Char('a'), c: 'a' };
    let repetition = ast::Repetition {
        span,
        op: ast::RepetitionOp::ZeroOrMore,
        greedy: true,
        ast: Box::new(ast::Ast::Literal(literal.clone())),
    };
    let group = ast::Group {
        span,
        kind: ast::GroupKind::NonCapturing,
        ast: Box::new(ast::Ast::Repetition(repetition)),
    };

    translator.flags.set(Flags {
        case_insensitive: Some(false),
        multi_line: Some(true),
        dot_matches_new_line: Some(false),
        swap_greed: Some(true),
        unicode: Some(true),
    });

    let group_ast = ast::Ast::Group(group);
    translator.visit_post(&group_ast).unwrap();
}

