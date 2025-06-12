// Answer 0

#[test]
fn test_into_ast_with_single_literal() {
    struct Position {
        offset: usize,
    }

    impl Position {
        pub fn new(offset: usize) -> Self {
            Position { offset }
        }
    }

    struct Literal {
        character: char,
        span: Span,
    }

    impl Literal {
        pub fn new(character: char, span: Span) -> Self {
            Literal { character, span }
        }
    }

    let span = Span {
        start: Position::new(0),
        end: Position::new(1),
    };
    
    let literal = Ast::Literal(Literal::new('a', span));
    let concat = Concat {
        span,
        asts: vec![literal],
    };

    concat.into_ast();
}

#[test]
fn test_into_ast_with_empty_concat() {
    let span = Span {
        start: Position::new(0),
        end: Position::new(0),
    };
    
    let concat = Concat {
        span,
        asts: vec![],
    };

    concat.into_ast();
}

#[test]
fn test_into_ast_with_two_elements() {
    let span = Span {
        start: Position::new(0),
        end: Position::new(2),
    };

    let literal_a = Ast::Literal(Literal::new('a', Span { start: Position::new(0), end: Position::new(1) }));
    let literal_b = Ast::Literal(Literal::new('b', Span { start: Position::new(1), end: Position::new(2) }));
    
    let concat = Concat {
        span,
        asts: vec![literal_a, literal_b],
    };

    concat.into_ast();
}

