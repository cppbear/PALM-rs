// Answer 0

#[test]
fn test_parse_with_ast_error() {
    struct TestAst;

    impl TestAst {
        fn parse(&self, pattern: &str) -> Result<ast::Ast> {
            Err(ast::Error::new("Test AST Error"))
        }
    }

    struct TestTranslator;

    impl TestTranslator {
        fn translate(&self, pattern: &str, ast: &ast::Ast) -> Result<hir::Hir> {
            Ok(hir::Hir::new())
        }
    }

    let mut parser = Parser {
        ast: TestAst,
        hir: TestTranslator,
    };

    let result = parser.parse("valid_pattern_that_causes_ast_error");
    assert!(result.is_err());
}

#[test]
fn test_parse_with_translate_error() {
    struct TestAst;

    impl TestAst {
        fn parse(&self, pattern: &str) -> Result<ast::Ast> {
            Ok(ast::Ast::Literal(ast::Literal::new("a")))
        }
    }

    struct TestTranslator;

    impl TestTranslator {
        fn translate(&self, pattern: &str, ast: &ast::Ast) -> Result<hir::Hir> {
            Err(hir::Error::new("Test Translate Error"))
        }
    }

    let mut parser = Parser {
        ast: TestAst,
        hir: TestTranslator,
    };

    let result = parser.parse("valid_pattern");
    assert!(result.is_err());
}

