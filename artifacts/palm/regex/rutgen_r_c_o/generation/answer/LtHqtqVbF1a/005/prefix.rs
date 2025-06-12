// Answer 0

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, Ast};
    use std::fmt::Write;

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = Repetition {
        span: Span::new(0, 10),
        op: RepetitionOp {
            kind: RepetitionKind::OneOrMore,
        },
        greedy: true,
        ast: Box::new(Ast::new()),
    };
    
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    
    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_one_or_more_non_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, Ast};
    use std::fmt::Write;

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = Repetition {
        span: Span::new(0, 10),
        op: RepetitionOp {
            kind: RepetitionKind::OneOrMore,
        },
        greedy: false,
        ast: Box::new(Ast::new()),
    };
    
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    
    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_one_or_more_with_range() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, Ast};
    use std::fmt::Write;

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = Repetition {
        span: Span::new(0, 10),
        op: RepetitionOp {
            kind: RepetitionKind::Range(RepetitionRange::Bounded(1, u32::MAX)),
        },
        greedy: true,
        ast: Box::new(Ast::new()),
    };
    
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    
    writer.fmt_repetition(&ast).unwrap();
}

