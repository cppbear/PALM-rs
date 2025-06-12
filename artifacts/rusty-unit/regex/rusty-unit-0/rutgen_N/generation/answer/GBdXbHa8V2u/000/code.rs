// Answer 0

#[test]
fn test_nest_limit_zero() {
    struct MockAst {
        limit: u32,
    }

    impl MockAst {
        fn nest_limit(&mut self, limit: u32) {
            self.limit = limit;
        }
    }

    struct ParserBuilder {
        ast: MockAst,
    }

    let mut parser = ParserBuilder {
        ast: MockAst { limit: 0 },
    };

    let result = parser.nest_limit(0);
    assert_eq!(result.ast.limit, 0);
}

#[test]
fn test_nest_limit_non_zero() {
    struct MockAst {
        limit: u32,
    }

    impl MockAst {
        fn nest_limit(&mut self, limit: u32) {
            self.limit = limit;
        }
    }

    struct ParserBuilder {
        ast: MockAst,
    }

    let mut parser = ParserBuilder {
        ast: MockAst { limit: 0 },
    };

    let result = parser.nest_limit(10);
    assert_eq!(result.ast.limit, 10);
}

#[test]
fn test_nest_limit_edge_case() {
    struct MockAst {
        limit: u32,
    }

    impl MockAst {
        fn nest_limit(&mut self, limit: u32) {
            self.limit = limit;
        }
    }

    struct ParserBuilder {
        ast: MockAst,
    }

    let mut parser = ParserBuilder {
        ast: MockAst { limit: 0 },
    };

    let result = parser.nest_limit(u32::MAX);
    assert_eq!(result.ast.limit, u32::MAX);
}

