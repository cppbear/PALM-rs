// Answer 0

#[derive(Default)]
struct Parser {
    nest_limit: u32,
}

struct Context {
    depth: u32,
    p: ParserWrapper,
}

struct ParserWrapper {
    parser: Parser,
}

impl ParserWrapper {
    fn parser(&self) -> &Parser {
        &self.parser
    }
    
    fn error(&self, _span: Span, error_kind: ast::ErrorKind) -> Result<()> {
        Err(error_kind.into())
    }
}

#[derive(Clone)]
struct Span;

#[derive(Debug)]
enum ast {
    // Mock representation of ast::ErrorKind 
    ErrorKind,
}

impl From<ast::ErrorKind> for Result<()> {
    fn from(_kind: ast::ErrorKind) -> Self {
        Err(())
    }
}

impl Context {
    fn increment_depth(&mut self, span: &Span) -> Result<()> {
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
#[should_panic]
fn test_increment_depth_exceeds_max() {
    let span = Span;
    let mut context = Context {
        depth: u32::MAX,
        p: ParserWrapper { parser: Parser { nest_limit: 10 } },
    };
    let result = context.increment_depth(&span);
    assert!(result.is_err());
}

#[test]
fn test_increment_depth_within_limit() {
    let span = Span;
    let mut context = Context {
        depth: 5,
        p: ParserWrapper { parser: Parser { nest_limit: 10 } },
    };
    let result = context.increment_depth(&span);
    assert!(result.is_ok());
    assert_eq!(context.depth, 6);
}

#[test]
#[should_panic]
fn test_increment_depth_exceeds_limit() {
    let span = Span;
    let mut context = Context {
        depth: 10,
        p: ParserWrapper { parser: Parser { nest_limit: 10 } },
    };
    let result = context.increment_depth(&span);
    assert!(result.is_err());
}

#[test]
fn test_increment_depth_empty() {
    let span = Span;
    let mut context = Context {
        depth: 0,
        p: ParserWrapper { parser: Parser { nest_limit: 10 } },
    };
    let result = context.increment_depth(&span);
    assert!(result.is_ok());
    assert_eq!(context.depth, 1);
}

