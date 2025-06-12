// Answer 0

#[test]
fn test_span_empty() {
    struct AstEmpty { span: Span }
    impl Ast for AstEmpty {
        fn span(&self) -> &Span {
            &self.span
        }
    }
    
    let empty_ast = AstEmpty { span: Span { start: 0, end: 0 } };
    assert_eq!(empty_ast.span(), &Span { start: 0, end: 0 });
}

#[test]
fn test_span_flags() {
    struct AstFlags { span: Span }
    struct Flags { span: Span }
    
    impl Ast for AstFlags {
        fn span(&self) -> &Span {
            &self.span
        }
    }

    let flags_ast = AstFlags { span: Flags { span: Span { start: 1, end: 5 } }.span };
    assert_eq!(flags_ast.span(), &Span { start: 1, end: 5 });
}

#[test]
fn test_span_literal() {
    struct AstLiteral { span: Span }
    struct Literal { span: Span }

    impl Ast for AstLiteral {
        fn span(&self) -> &Span {
            &self.span
        }
    }

    let literal_ast = AstLiteral { span: Literal { span: Span { start: 2, end: 6 } }.span };
    assert_eq!(literal_ast.span(), &Span { start: 2, end: 6 });
}

#[test]
fn test_span_dot() {
    struct AstDot { span: Span }
    
    impl Ast for AstDot {
        fn span(&self) -> &Span {
            &self.span
        }
    }
    
    let dot_ast = AstDot { span: Span { start: 3, end: 7 } };
    assert_eq!(dot_ast.span(), &Span { start: 3, end: 7 });
}

#[test]
fn test_span_assertion() {
    struct AstAssertion { span: Span }
    struct Assertion { span: Span }
    
    impl Ast for AstAssertion {
        fn span(&self) -> &Span {
            &self.span
        }
    }

    let assertion_ast = AstAssertion { span: Assertion { span: Span { start: 4, end: 8 } }.span };
    assert_eq!(assertion_ast.span(), &Span { start: 4, end: 8 });
}

#[test]
fn test_span_class() {
    struct AstClass { span: Span }
    
    impl Ast for AstClass {
        fn span(&self) -> &Span {
            &self.span
        }
    }
    
    let class_ast = AstClass { span: Span { start: 5, end: 9 } };
    assert_eq!(class_ast.span(), &Span { start: 5, end: 9 });
}

#[test]
fn test_span_repetition() {
    struct AstRepetition { span: Span }
    
    impl Ast for AstRepetition {
        fn span(&self) -> &Span {
            &self.span
        }
    }
    
    let repetition_ast = AstRepetition { span: Span { start: 6, end: 10 } };
    assert_eq!(repetition_ast.span(), &Span { start: 6, end: 10 });
}

#[test]
fn test_span_group() {
    struct AstGroup { span: Span }
    
    impl Ast for AstGroup {
        fn span(&self) -> &Span {
            &self.span
        }
    }
    
    let group_ast = AstGroup { span: Span { start: 7, end: 11 } };
    assert_eq!(group_ast.span(), &Span { start: 7, end: 11 });
}

#[test]
fn test_span_alternation() {
    struct AstAlternation { span: Span }
    
    impl Ast for AstAlternation {
        fn span(&self) -> &Span {
            &self.span
        }
    }
    
    let alternation_ast = AstAlternation { span: Span { start: 8, end: 12 } };
    assert_eq!(alternation_ast.span(), &Span { start: 8, end: 12 });
}

#[test]
fn test_span_concat() {
    struct AstConcat { span: Span }
    
    impl Ast for AstConcat {
        fn span(&self) -> &Span {
            &self.span
        }
    }
    
    let concat_ast = AstConcat { span: Span { start: 9, end: 13 } };
    assert_eq!(concat_ast.span(), &Span { start: 9, end: 13 });
}

