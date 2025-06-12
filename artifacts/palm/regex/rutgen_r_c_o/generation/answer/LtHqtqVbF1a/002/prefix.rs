// Answer 0

#[test]
fn test_fmt_repetition_range_bounded() {
    let mut buffer = String::new();
    let repetition = ast::Repetition {
        span: Span::default(), // Assuming default provides a valid span
        op: ast::RepetitionOp { 
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(5, 10)) 
        },
        greedy: true,
        ast: Box::new(ast::Ast::default()), // Assuming default provides a valid AST
    };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut buffer };
    writer.fmt_repetition(&repetition).unwrap();
}

#[test]
fn test_fmt_repetition_range_at_least() {
    let mut buffer = String::new();
    let repetition = ast::Repetition {
        span: Span::default(), // Assuming valid default
        op: ast::RepetitionOp { 
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(3)) 
        },
        greedy: true,
        ast: Box::new(ast::Ast::default()), // Assuming valid default
    };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut buffer };
    writer.fmt_repetition(&repetition).unwrap();
}

#[test]
fn test_fmt_repetition_range_exactly() {
    let mut buffer = String::new();
    let repetition = ast::Repetition {
        span: Span::default(), // Assuming valid default
        op: ast::RepetitionOp { 
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(7)) 
        },
        greedy: true,
        ast: Box::new(ast::Ast::default()), // Assuming valid default
    };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut buffer };
    writer.fmt_repetition(&repetition).unwrap();
}

#[test]
fn test_fmt_repetition_range_bounded_max() {
    let mut buffer = String::new();
    let repetition = ast::Repetition {
        span: Span::default(), // Assuming valid default
        op: ast::RepetitionOp { 
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(0, 1000)) 
        },
        greedy: true,
        ast: Box::new(ast::Ast::default()), // Assuming valid default
    };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut buffer };
    writer.fmt_repetition(&repetition).unwrap();
}

