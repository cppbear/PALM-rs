// Answer 0

#[test]
fn test_parse_valid_regex() {
    struct Parser {
        ast: AstParser,
        hir: HirTranslator,
    }

    struct AstParser;
    struct HirTranslator;

    impl AstParser {
        fn parse(&self, pattern: &str) -> Result<Ast, String> {
            // Simulate parsing logic for valid patterns
            Ok(Ast)
        }
    }

    impl HirTranslator {
        fn translate(&self, pattern: &str, _ast: &Ast) -> Result<Hir, String> {
            // Simulate translation for valid patterns
            Ok(Hir)
        }
    }

    impl Parser {
        pub fn parse(&mut self, pattern: &str) -> Result<Hir, String> {
            let ast = self.ast.parse(pattern)?;
            let hir = self.hir.translate(pattern, &ast)?;
            Ok(hir)
        }
    }

    struct Ast;
    struct Hir;

    let mut parser = Parser {
        ast: AstParser,
        hir: HirTranslator,
    };

    let result = parser.parse("a*b+").unwrap();
    assert!(result.is_instance_of::<Hir>());
}

#[test]
#[should_panic(expected = "Parse error")]
fn test_parse_invalid_regex() {
    struct Parser {
        ast: AstParser,
        hir: HirTranslator,
    }

    struct AstParser;
    struct HirTranslator;

    impl AstParser {
        fn parse(&self, pattern: &str) -> Result<Ast, String> {
            if pattern == "invalid[" {
                Err("Parse error".into())
            } else {
                Ok(Ast)
            }
        }
    }

    impl HirTranslator {
        fn translate(&self, _pattern: &str, _ast: &Ast) -> Result<Hir, String> {
            Ok(Hir)
        }
    }

    impl Parser {
        pub fn parse(&mut self, pattern: &str) -> Result<Hir, String> {
            let ast = self.ast.parse(pattern)?;
            let hir = self.hir.translate(pattern, &ast)?;
            Ok(hir)
        }
    }

    struct Ast;
    struct Hir;

    let mut parser = Parser {
        ast: AstParser,
        hir: HirTranslator,
    };

    let _ = parser.parse("invalid[").unwrap(); // This should panic
}

