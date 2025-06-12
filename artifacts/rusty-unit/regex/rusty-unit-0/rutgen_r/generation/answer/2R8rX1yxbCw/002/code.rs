// Answer 0

#[test]
fn test_parse_success() {
    struct TestAst;
    struct TestHir;

    impl TestAst {
        fn parse(&self, pattern: &str) -> Result<String> {
            Ok(pattern.to_string())
        }
    }

    impl TestHir {
        fn translate(&self, _pattern: &str, _ast: &String) -> Result<String> {
            Ok("translated".to_string())
        }
    }

    struct Parser {
        ast: TestAst,
        hir: TestHir,
    }

    let mut parser = Parser {
        ast: TestAst,
        hir: TestHir,
    };

    let result = parser.parse("a*b");
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_ast_parse_fail() {
    struct TestAst;
    struct TestHir;

    impl TestAst {
        fn parse(&self, _pattern: &str) -> Result<String> {
            Err("parse error".into())
        }
    }

    impl TestHir {
        fn translate(&self, _pattern: &str, _ast: &String) -> Result<String> {
            Ok("translated".to_string())
        }
    }

    struct Parser {
        ast: TestAst,
        hir: TestHir,
    }

    let mut parser = Parser {
        ast: TestAst,
        hir: TestHir,
    };

    let _result = parser.parse("invalid_pattern");
}

#[test]
#[should_panic]
fn test_parse_hir_translate_fail() {
    struct TestAst;
    struct TestHir;

    impl TestAst {
        fn parse(&self, pattern: &str) -> Result<String> {
            Ok(pattern.to_string())
        }
    }

    impl TestHir {
        fn translate(&self, _pattern: &str, _ast: &String) -> Result<String> {
            Err("translation error".into())
        }
    }

    struct Parser {
        ast: TestAst,
        hir: TestHir,
    }

    let mut parser = Parser {
        ast: TestAst,
        hir: TestHir,
    };

    let _result = parser.parse("x?y");
}

