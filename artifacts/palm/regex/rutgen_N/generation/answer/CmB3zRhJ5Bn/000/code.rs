// Answer 0

#[test]
fn test_into_ast_literal() {
    struct Primitive {
        // assuming a simple struct to represent a Literal
        kind: String,
    }
    
    struct Ast {
        // assuming a simple struct to represent an AST Literal
        kind: String,
    }
    
    impl Primitive {
        fn into_ast(self) -> Ast {
            match self {
                Primitive { kind } => Ast { kind },
            }
        }
    }

    let primitive = Primitive { kind: String::from("example") };
    let ast = primitive.into_ast();
    assert_eq!(ast.kind, "example");
}

#[test]
fn test_into_ast_assertion() {
    struct Assertion {
        // assuming a simple struct to represent an Assertion
        kind: String,
    }
    
    struct Primitive {
        assertion: Assertion,
    }
    
    struct Ast {
        // assuming a simple struct to represent an AST Assertion
        kind: String,
    }
    
    impl Primitive {
        fn into_ast(self) -> Ast {
            match self {
                Primitive { assertion } => Ast { kind: assertion.kind },
            }
        }
    }

    let assertion = Assertion { kind: String::from("start") };
    let primitive = Primitive { assertion };
    let ast = primitive.into_ast();
    assert_eq!(ast.kind, "start");
}

#[test]
fn test_into_ast_dot() {
    struct Span {
        // assuming a simple struct to represent a Span
        range: (u32, u32),
    }
    
    struct Primitive {
        dot: Span,
    }
    
    struct Ast {
        // assuming a simple struct to represent an AST Dot
        range: (u32, u32),
    }
    
    impl Primitive {
        fn into_ast(self) -> Ast {
            match self {
                Primitive { dot } => Ast { range: dot.range },
            }
        }
    }

    let span = Span { range: (1, 2) };
    let primitive = Primitive { dot: span };
    let ast = primitive.into_ast();
    assert_eq!(ast.range, (1, 2));
}

#[test]
fn test_into_ast_perl_class() {
    struct PerlClass {
        // assuming a simple struct to represent a Perl Class
        name: String,
    }
    
    struct Class {
        perl: PerlClass,
    }
    
    struct Primitive {
        perl: PerlClass,
    }
    
    struct Ast {
        // assuming a simple struct to represent an AST Class
        class_type: Class,
    }
    
    impl Primitive {
        fn into_ast(self) -> Ast {
            match self {
                Primitive { perl } => Ast { class_type: Class { perl } },
            }
        }
    }

    let perl_class = PerlClass { name: String::from("perl_class") };
    let primitive = Primitive { perl: perl_class };
    let ast = primitive.into_ast();
    assert_eq!(ast.class_type.perl.name, "perl_class");
}

#[test]
fn test_into_ast_unicode_class() {
    struct UnicodeClass {
        // assuming a simple struct to represent a Unicode Class
        name: String,
    }
    
    struct Class {
        unicode: UnicodeClass,
    }
    
    struct Primitive {
        unicode: UnicodeClass,
    }
    
    struct Ast {
        // assuming a simple struct to represent an AST Class
        class_type: Class,
    }
    
    impl Primitive {
        fn into_ast(self) -> Ast {
            match self {
                Primitive { unicode } => Ast { class_type: Class { unicode } },
            }
        }
    }

    let unicode_class = UnicodeClass { name: String::from("unicode_class") };
    let primitive = Primitive { unicode: unicode_class };
    let ast = primitive.into_ast();
    assert_eq!(ast.class_type.unicode.name, "unicode_class");
}

