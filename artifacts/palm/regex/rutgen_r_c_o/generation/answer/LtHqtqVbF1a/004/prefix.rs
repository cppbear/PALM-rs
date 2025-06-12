// Answer 0

#[test]
fn test_fmt_repetition_range_bounded() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(1, 3)) },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_at_least() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(2)) },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_exactly() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(5)) },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
}

