// Answer 0

#[test]
fn test_fmt_repetition_zero_or_more_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, Ast};

    let span = Span { start: 1, end: 10 }; 
    let repetition = Repetition {
        span,
        op: RepetitionOp { kind: RepetitionKind::ZeroOrMore },
        greedy: true,
        ast: Box::new(Ast::new()), // Assuming a valid Ast structure is required
    };

    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: String::new() };

    writer.fmt_repetition(&repetition).unwrap();
}

#[test]
fn test_fmt_repetition_zero_or_more_not_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, Ast};

    let span = Span { start: 1, end: 10 }; 
    let repetition = Repetition {
        span,
        op: RepetitionOp { kind: RepetitionKind::ZeroOrMore },
        greedy: false,
        ast: Box::new(Ast::new()), // Assuming a valid Ast structure is required
    };

    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: String::new() };

    writer.fmt_repetition(&repetition).unwrap();
}

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, Ast};

    let span = Span { start: 1, end: 10 }; 
    let repetition = Repetition {
        span,
        op: RepetitionOp { kind: RepetitionKind::OneOrMore },
        greedy: true,
        ast: Box::new(Ast::new()), // Assuming a valid Ast structure is required
    };

    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: String::new() };

    writer.fmt_repetition(&repetition).unwrap();
}

#[test]
fn test_fmt_repetition_one_or_more_not_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, Ast};

    let span = Span { start: 1, end: 10 }; 
    let repetition = Repetition {
        span,
        op: RepetitionOp { kind: RepetitionKind::OneOrMore },
        greedy: false,
        ast: Box::new(Ast::new()), // Assuming a valid Ast structure is required
    };

    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: String::new() };

    writer.fmt_repetition(&repetition).unwrap();
}

#[test]
fn test_fmt_repetition_zero_or_one_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, Ast};

    let span = Span { start: 1, end: 10 }; 
    let repetition = Repetition {
        span,
        op: RepetitionOp { kind: RepetitionKind::ZeroOrOne },
        greedy: true,
        ast: Box::new(Ast::new()), // Assuming a valid Ast structure is required
    };

    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: String::new() };

    writer.fmt_repetition(&repetition).unwrap();
}

#[test]
fn test_fmt_repetition_zero_or_one_not_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, Ast};

    let span = Span { start: 1, end: 10 }; 
    let repetition = Repetition {
        span,
        op: RepetitionOp { kind: RepetitionKind::ZeroOrOne },
        greedy: false,
        ast: Box::new(Ast::new()), // Assuming a valid Ast structure is required
    };

    let mut printer = Printer { _priv: () };
    let writer = Writer { printer: &mut printer, wtr: String::new() };

    writer.fmt_repetition(&repetition).unwrap();
}

