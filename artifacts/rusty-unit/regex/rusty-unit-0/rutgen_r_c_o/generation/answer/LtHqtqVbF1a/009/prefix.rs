// Answer 0

#[test]
fn test_fmt_repetition_zero_or_one_greedy() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: true,
        ast: Box::new(ast::Ast::empty()),
    };
    
    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_zero_or_more_greedy() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: true,
        ast: Box::new(ast::Ast::empty()),
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let ast = ast::Repetition {
        span: Span::new(0, 1),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: true,
        ast: Box::new(ast::Ast::empty()),
    };

    writer.fmt_repetition(&ast).unwrap();
}

