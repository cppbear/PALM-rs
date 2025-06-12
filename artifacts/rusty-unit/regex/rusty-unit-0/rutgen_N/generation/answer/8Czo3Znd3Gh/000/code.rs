// Answer 0

#[test]
fn test_is_empty_empty_ast() {
    struct EmptyAst;
    
    impl EmptyAst {
        fn is_empty(&self) -> bool {
            match *self {
                EmptyAst => true,
                _ => false,
            }
        }
    }
    
    let empty_ast = EmptyAst;
    assert!(empty_ast.is_empty());
}

#[test]
fn test_is_empty_non_empty_ast() {
    struct NonEmptyAst;

    impl NonEmptyAst {
        fn is_empty(&self) -> bool {
            match *self {
                NonEmptyAst => true,
                _ => false,
            }
        }
    }

    let non_empty_ast = NonEmptyAst;
    assert!(!non_empty_ast.is_empty());
}

