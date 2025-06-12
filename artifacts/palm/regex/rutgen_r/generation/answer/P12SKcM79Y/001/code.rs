// Answer 0

#[test]
fn test_build_function_with_valid_configuration() {
    struct Ast {
        // Assuming `Ast` requires some field. Add fields as needed.
    }
    
    impl Ast {
        fn build(&self) -> String {
            // Simulated build process
            "AST_Built".to_string() // Expected output
        }
    }

    struct Hir {
        // Assuming `Hir` requires some field. Add fields as needed.
    }
    
    impl Hir {
        fn build(&self) -> String {
            // Simulated build process
            "HIR_Built".to_string() // Expected output
        }
    }

    struct Parser {
        ast: String,
        hir: String,
    }

    struct Config {
        ast: Ast,
        hir: Hir,
    }

    impl Config {
        pub fn build(&self) -> Parser {
            Parser {
                ast: self.ast.build(),
                hir: self.hir.build(),
            }
        }
    }

    let ast = Ast {};
    let hir = Hir {};
    let config = Config { ast, hir };

    let parser = config.build();
    assert_eq!(parser.ast, "AST_Built");
    assert_eq!(parser.hir, "HIR_Built");
}

#[test]
#[should_panic]
fn test_build_function_with_invalid_ast() {
    // This would require a specific way to trigger a panic within the Ast build process.
    struct Ast;

    impl Ast {
        fn build(&self) -> String {
            panic!("Invalid AST configuration");
        }
    }

    struct Hir {
        // Assuming Hir can still function correctly.
    }
    
    impl Hir {
        fn build(&self) -> String {
            "HIR_Built".to_string() // Expected output
        }
    }

    struct Parser {
        ast: String,
        hir: String,
    }

    struct Config {
        ast: Ast,
        hir: Hir,
    }
    
    impl Config {
        pub fn build(&self) -> Parser {
            Parser {
                ast: self.ast.build(),
                hir: self.hir.build(),
            }
        }
    }

    let ast = Ast {};
    let hir = Hir {};
    let config = Config { ast, hir };
    
    // This should panic
    let _ = config.build();
}

#[test]
#[should_panic]
fn test_build_function_with_invalid_hir() {
    // This would require a specific way to trigger a panic within the Hir build process.
    struct Ast {
        // Assuming Ast can still function correctly.
    }

    impl Ast {
        fn build(&self) -> String {
            "AST_Built".to_string() // Expected output
        }
    }

    struct Hir;

    impl Hir {
        fn build(&self) -> String {
            panic!("Invalid HIR configuration");
        }
    }

    struct Parser {
        ast: String,
        hir: String,
    }

    struct Config {
        ast: Ast,
        hir: Hir,
    }
    
    impl Config {
        pub fn build(&self) -> Parser {
            Parser {
                ast: self.ast.build(),
                hir: self.hir.build(),
            }
        }
    }

    let ast = Ast {};
    let hir = Hir {};
    let config = Config { ast, hir };
    
    // This should panic
    let _ = config.build();
}

