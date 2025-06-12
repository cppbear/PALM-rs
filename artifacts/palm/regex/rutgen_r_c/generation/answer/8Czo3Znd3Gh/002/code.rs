// Answer 0

#[test]
fn test_ast_is_empty_empty_variant() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Position {
        offset: usize,
    }
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span {
        start: Position,
        end: Position,
    }
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Ast {
        Empty(Span),
        // Other variants omitted for conciseness
    }

    impl Ast {
        pub fn is_empty(&self) -> bool {
            match *self {
                Ast::Empty(_) => true,
                _ => false,
            }
        }
    }

    let position = Position { offset: 0 };
    let span = Span { start: position.clone(), end: position };
    let ast_empty = Ast::Empty(span);
    
    assert!(ast_empty.is_empty());
}

