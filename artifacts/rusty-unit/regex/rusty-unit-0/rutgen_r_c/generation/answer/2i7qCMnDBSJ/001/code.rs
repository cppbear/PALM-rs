// Answer 0

#[test]
fn test_has_subexprs_concat() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span {
        start: usize,
        end: usize,
    }
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Repetition {
        span: Span,
        op: String,
        greedy: bool,
        ast: Box<Ast>,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Group {
        span: Span,
        kind: String,
        hir: Box<Ast>,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Alternation {
        span: Span,
        asts: Vec<Ast>,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Literal {
        span: Span,
        kind: String,
        c: char,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Ast {
        Empty(Span),
        Flags,
        Literal(Literal),
        Dot(Span),
        Assertion,
        Class,
        Repetition(Repetition),
        Group(Group),
        Alternation(Alternation),
        Concat { span: Span, asts: Vec<Ast> },
    }
    
    impl Ast {
        pub fn has_subexprs(&self) -> bool {
            match self {
                Ast::Empty(_) 
                | Ast::Flags 
                | Ast::Literal(_) 
                | Ast::Dot(_) 
                | Ast::Assertion => false,
                Ast::Class 
                | Ast::Repetition(_) 
                | Ast::Group(_) 
                | Ast::Alternation(_) 
                | Ast::Concat { .. } => true,
            }
        }
    }

    let span = Span { start: 0, end: 10 };
    let sub_expr1 = Ast::Literal(Literal { span: span.clone(), kind: String::from("char"), c: 'a' });
    let sub_expr2 = Ast::Group(Group { span: span.clone(), kind: String::from("capture"), hir: Box::new(sub_expr1) });
    
    let concat_ast = Ast::Concat { span, asts: vec![sub_expr2] };

    assert!(concat_ast.has_subexprs());
}

#[test]
fn test_has_subexprs_empty() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span {
        start: usize,
        end: usize,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Ast {
        Empty(Span),
        Literal,
        // Other variants omitted for brevity
    }

    impl Ast {
        pub fn has_subexprs(&self) -> bool {
            match self {
                Ast::Empty(_) => false,
                // Add other match arms as needed
            }
        }
    }

    let span = Span { start: 0, end: 0 };
    let empty_ast = Ast::Empty(span);

    assert!(!empty_ast.has_subexprs());
}

