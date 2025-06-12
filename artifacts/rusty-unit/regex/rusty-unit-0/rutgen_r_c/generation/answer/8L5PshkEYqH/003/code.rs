// Answer 0

#[test]
fn test_into_ast_empty() {
    struct TestPosition;
    #[derive(Clone, Copy, Eq, PartialEq)]
    struct TestSpan {
        start: TestPosition,
        end: TestPosition,
    }

    let test_span = TestSpan { start: TestPosition, end: TestPosition };
    let alternation = Alternation {
        span: test_span,
        asts: Vec::new(),
    };
    
    if let Ast::Empty(span) = alternation.clone().into_ast() {
        assert_eq!(span, test_span);
    } else {
        panic!("Expected Ast::Empty");
    }
}

#[test]
fn test_into_ast_single() {
    struct TestPosition;
    #[derive(Clone, Copy, Eq, PartialEq)]
    struct TestSpan {
        start: TestPosition,
        end: TestPosition,
    }

    let test_span = TestSpan { start: TestPosition, end: TestPosition };
    let single_ast = Ast::Literal(Literal);
    let alternation = Alternation {
        span: test_span,
        asts: vec![single_ast.clone()],
    };
    
    assert_eq!(alternation.clone().into_ast(), single_ast);
}

#[test]
fn test_into_ast_multiple() {
    struct TestPosition;
    #[derive(Clone, Copy, Eq, PartialEq)]
    struct TestSpan {
        start: TestPosition,
        end: TestPosition,
    }

    let test_span = TestSpan { start: TestPosition, end: TestPosition };
    let alternation = Alternation {
        span: test_span,
        asts: vec![Ast::Literal(Literal), Ast::Dot(test_span)],
    };
    
    if let Ast::Alternation(result) = alternation.clone().into_ast() {
        assert_eq!(result.span, test_span);
        assert_eq!(result.asts.len(), 2);
    } else {
        panic!("Expected Ast::Alternation");
    }
}

