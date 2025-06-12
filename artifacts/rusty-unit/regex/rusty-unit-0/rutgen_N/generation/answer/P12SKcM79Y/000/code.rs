// Answer 0

#[test]
fn test_build_with_valid_configuration() {
    struct Config {
        ast: AstConfig,
        hir: HirConfig,
    }

    struct AstConfig;
    struct HirConfig;

    impl AstConfig {
        fn build(&self) -> String {
            "AST".to_string()
        }
    }

    impl HirConfig {
        fn build(&self) -> String {
            "HIR".to_string()
        }
    }

    impl Config {
        fn build(&self) -> Parser {
            Parser {
                ast: self.ast.build(),
                hir: self.hir.build(),
            }
        }
    }

    struct Parser {
        ast: String,
        hir: String,
    }

    let config = Config {
        ast: AstConfig,
        hir: HirConfig,
    };

    let parser = config.build();
    assert_eq!(parser.ast, "AST");
    assert_eq!(parser.hir, "HIR");
}

#[test]
fn test_build_with_empty_configuration() {
    struct EmptyConfig {
        ast: EmptyAstConfig,
        hir: EmptyHirConfig,
    }

    struct EmptyAstConfig;
    struct EmptyHirConfig;

    impl EmptyAstConfig {
        fn build(&self) -> String {
            "".to_string()
        }
    }

    impl EmptyHirConfig {
        fn build(&self) -> String {
            "".to_string()
        }
    }

    impl EmptyConfig {
        fn build(&self) -> Parser {
            Parser {
                ast: self.ast.build(),
                hir: self.hir.build(),
            }
        }
    }

    struct Parser {
        ast: String,
        hir: String,
    }

    let empty_config = EmptyConfig {
        ast: EmptyAstConfig,
        hir: EmptyHirConfig,
    };

    let parser = empty_config.build();
    assert_eq!(parser.ast, "");
    assert_eq!(parser.hir, "");
}

