// Answer 0

#[test]
fn test_into_ast_empty_concat() {
    struct DummyVisitor;

    #[derive(Clone)]
    struct Position(usize);

    #[derive(Clone)]
    struct SetFlags;

    #[derive(Clone)]
    struct Literal;

    #[derive(Clone)]
    struct Assertion;

    #[derive(Clone)]
    struct Class;

    #[derive(Clone)]
    struct Repetition;

    #[derive(Clone)]
    struct Group;

    #[derive(Clone)]
    struct Alternation;

    let start_position = Position(0);
    let end_position = Position(0);
    
    let span = Span {
        start: start_position,
        end: end_position,
    };

    let concat = Concat {
        span,
        asts: Vec::new(),
    };

    // When calling into_ast with an empty Concat, it should return Ast::Empty
    if let Ast::Empty(returned_span) = concat.into_ast() {
        assert_eq!(returned_span, concat.span);
    } else {
        panic!("Expected Ast::Empty");
    }
}

#[test]
fn test_into_ast_single_concat() {
    struct DummyVisitor;

    #[derive(Clone)]
    struct Position(usize);

    #[derive(Clone)]
    struct SetFlags;

    #[derive(Clone)]
    struct Literal;

    #[derive(Clone)]
    struct Assertion;

    #[derive(Clone)]
    struct Class;

    #[derive(Clone)]
    struct Repetition;

    #[derive(Clone)]
    struct Group;

    #[derive(Clone)]
    struct Alternation;

    let start_position = Position(0);
    let end_position = Position(5);
    
    let span = Span {
        start: start_position.clone(),
        end: end_position.clone(),
    };

    let ast_to_return = Ast::Literal(Literal);
    
    let concat = Concat {
        span,
        asts: vec![ast_to_return.clone()],
    };

    // When calling into_ast with a single AST in Concat, it should return that AST
    let returned_ast = concat.into_ast();
    assert_eq!(returned_ast, ast_to_return);
}

#[test]
fn test_into_ast_multiple_concat() {
    struct DummyVisitor;

    #[derive(Clone)]
    struct Position(usize);

    #[derive(Clone)]
    struct SetFlags;

    #[derive(Clone)]
    struct Literal;

    #[derive(Clone)]
    struct Assertion;

    #[derive(Clone)]
    struct Class;

    #[derive(Clone)]
    struct Repetition;

    #[derive(Clone)]
    struct Group;

    #[derive(Clone)]
    struct Alternation;

    let start_position = Position(0);
    let end_position = Position(10);
    
    let span = Span {
        start: start_position.clone(),
        end: end_position.clone(),
    };

    let ast1 = Ast::Literal(Literal);
    let ast2 = Ast::Dot(span.clone());
    
    let concat = Concat {
        span,
        asts: vec![ast1.clone(), ast2.clone()],
    };

    // When calling into_ast with multiple ASTs in Concat, it should return Ast::Concat
    let returned_ast = concat.into_ast();
    if let Ast::Concat(returned_concat) = returned_ast {
        assert_eq!(returned_concat.span, concat.span);
        assert_eq!(returned_concat.asts.len(), concat.asts.len());
    } else {
        panic!("Expected Ast::Concat");
    }
}

