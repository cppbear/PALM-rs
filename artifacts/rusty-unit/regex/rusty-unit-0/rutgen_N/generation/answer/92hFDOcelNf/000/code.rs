// Answer 0

#[derive(Default)]
struct Parser {
    nest_limit: u32,
}

struct Span;

struct Ast {
    depth: u32,
    p: ParserContext,
}

struct ParserContext {
    parser: Parser,
}

impl ParserContext {
    fn parser(&self) -> &Parser {
        &self.parser
    }

    fn error(&self, _span: Span, _kind: ast::ErrorKind) -> String {
        "Error".to_string()
    }
}

mod ast {
    #[derive(Debug)]
    pub enum ErrorKind {
        NestLimitExceeded(u32),
    }
}

impl Ast {
    fn increment_depth(&mut self, span: &Span) -> Result<(), String> {
        let new = self.depth.checked_add(1).ok_or_else(|| self.p.error(
            span.clone(),
            ast::ErrorKind::NestLimitExceeded(::std::u32::MAX),
        ))?;
        let limit = self.p.parser().nest_limit;
        if new > limit {
            return Err(self.p.error(
                span.clone(),
                ast::ErrorKind::NestLimitExceeded(limit),
            ));
        }
        self.depth = new;
        Ok(())
    }
}

#[test]
fn test_increment_depth_within_limit() {
    let span = Span;
    let mut ast = Ast {
        depth: 0,
        p: ParserContext {
            parser: Parser { nest_limit: 5 },
        },
    };
    
    assert_eq!(ast.increment_depth(&span), Ok(()));
    assert_eq!(ast.depth, 1);
}

#[test]
fn test_increment_depth_exceeds_limit() {
    let span = Span;
    let mut ast = Ast {
        depth: 5,
        p: ParserContext {
            parser: Parser { nest_limit: 5 },
        },
    };

    assert_eq!(ast.increment_depth(&span), Err("Error".to_string()));
    assert_eq!(ast.depth, 5);
}

#[test]
fn test_increment_depth_at_max_u32() {
    let span = Span;
    let mut ast = Ast {
        depth: std::u32::MAX,
        p: ParserContext {
            parser: Parser { nest_limit: std::u32::MAX },
        },
    };

    assert_eq!(ast.increment_depth(&span), Err("Error".to_string()));
}

