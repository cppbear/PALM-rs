// Answer 0

#[test]
fn test_visit_post_repetition_greedy() {
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let repetition = ast::Repetition {
        span: Span { start: 0, end: 1 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: true,
        ast: Box::new(ast::Ast::Empty(Span { start: 0, end: 0 })),
    };
    visitor.visit_post(&ast::Ast::Repetition(repetition));
}

#[test]
fn test_visit_post_repetition_not_greedy() {
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let repetition = ast::Repetition {
        span: Span { start: 1, end: 2 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: false,
        ast: Box::new(ast::Ast::Literal(ast::Literal { span: Span { start: 1, end: 1 }, kind: ast::LiteralKind::Verbatim, c: 'a' })),
    };
    visitor.visit_post(&ast::Ast::Repetition(repetition));
}

#[test]
fn test_visit_post_repetition_range_greedy() {
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let repetition = ast::Repetition {
        span: Span { start: 2, end: 5 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange { start: 2, end: 4 }) },
        greedy: true,
        ast: Box::new(ast::Ast::Assertion(ast::Assertion { span: Span { start: 2, end: 2 }, kind: ast::AssertionKind::WordBoundary })),
    };
    visitor.visit_post(&ast::Ast::Repetition(repetition));
}

#[test]
fn test_visit_post_repetition_range_not_greedy() {
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let repetition = ast::Repetition {
        span: Span { start: 3, end: 6 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange { start: 1, end: 3 }) },
        greedy: false,
        ast: Box::new(ast::Ast::Group(ast::Group { span: Span { start: 3, end: 3 }, kind: ast::GroupKind::Capturing, ast: Box::new(ast::Ast::Dot(Span { start: 3, end: 3 })) })),
    };
    visitor.visit_post(&ast::Ast::Repetition(repetition));
}

