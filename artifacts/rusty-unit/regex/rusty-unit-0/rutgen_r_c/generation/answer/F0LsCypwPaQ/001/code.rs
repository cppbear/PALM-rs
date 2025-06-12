// Answer 0

#[test]
fn test_fmt_with_empty_ast() {
    use std::fmt::Write;

    let empty_span = Span { start: Position::new(0), end: Position::new(0) };
    let empty_ast = Ast::Empty(empty_span);
    let mut output = String::new();
    
    assert!(write!(&mut output, "{}", empty_ast).is_ok());
    assert_eq!(output, ""); // Expecting it to print nothing
}

#[test]
fn test_fmt_with_literal_ast() {
    use std::fmt::Write;

    let span = Span { start: Position::new(0), end: Position::new(1) };
    let literal = Literal {
        span,
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    };
    let ast = Ast::Literal(literal);
    let mut output = String::new();
    
    assert!(write!(&mut output, "{}", ast).is_ok());
    // Assuming the print method formats it as a character
    assert_eq!(output, "a");
}

#[test]
fn test_fmt_with_repetition_ast() {
    use std::fmt::Write;

    let span = Span { start: Position::new(0), end: Position::new(3) };
    let literal = Literal {
        span,
        kind: LiteralKind::Unicode('b'),
        c: 'b',
    };
    let repetition = Repetition {
        span,
        op: RepetitionOp::ZeroOrMore,
        greedy: true,
        ast: Box::new(Ast::Literal(literal)),
    };
    let ast = Ast::Repetition(repetition);
    let mut output = String::new();
    
    assert!(write!(&mut output, "{}", ast).is_ok());
    // Assuming the expected non-detailed output would have some indication of repetition
    assert!(output.contains("*")); // Checking for the repetition character
}

#[test]
fn test_fmt_with_group_ast() {
    use std::fmt::Write;

    let span = Span { start: Position::new(0), end: Position::new(5) };
    let literal = Literal {
        span,
        kind: LiteralKind::Unicode('c'),
        c: 'c',
    };
    let group = Group {
        span,
        kind: GroupKind::Capturing(0),
        ast: Box::new(Ast::Literal(literal)),
    };
    let ast = Ast::Group(group);
    let mut output = String::new();
    
    assert!(write!(&mut output, "{}", ast).is_ok());
    // Assuming it has some indication of grouping in the output
    assert!(output.contains("(") && output.contains(")")); // Expecting parentheses for a group
}

#[test]
fn test_fmt_with_concatenation_ast() {
    use std::fmt::Write;

    let span = Span { start: Position::new(0), end: Position::new(4) };
    let literal_a = Literal {
        span,
        kind: LiteralKind::Unicode('d'),
        c: 'd',
    };
    let literal_b = Literal {
        span,
        kind: LiteralKind::Unicode('e'),
        c: 'e',
    };
    let concat = Concat {
        span,
        asts: vec![
            Box::new(Ast::Literal(literal_a)),
            Box::new(Ast::Literal(literal_b))
        ],
    };
    let ast = Ast::Concat(concat);
    let mut output = String::new();
    
    assert!(write!(&mut output, "{}", ast).is_ok());
    // The output should reflect concatenation
    assert_eq!(output, "de"); // Expecting concatenated output
}

