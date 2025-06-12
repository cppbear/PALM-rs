// Answer 0

#[derive(Clone)]
struct CaptureName {
    name: String,
    span: Span,
}

#[derive(Default)]
struct Parser {
    capture_names: std::cell::RefCell<Vec<CaptureName>>,
}

impl Parser {
    fn capture_names(&self) -> &std::cell::RefCell<Vec<CaptureName>> {
        &self.capture_names
    }
}

struct AstParser {
    parser: Parser,
}

impl AstParser {
    fn parser(&self) -> &Parser {
        &self.parser
    }

    fn error(&self, span: Span, error: ast::ErrorKind) -> ast::Error {
        // Assume some error construction here
        ast::Error::new(span, error)
    }

    fn add_capture_name(&self, cap: &CaptureName) -> Result<()> {
        let mut names = self.parser().capture_names.borrow_mut();
        match names.binary_search_by_key(
            &cap.name.as_str(),
            |c| c.name.as_str(),
        ) {
            Err(i) => {
                names.insert(i, cap.clone());
                Ok(())
            }
            Ok(i) => {
                Err(self.error(cap.span, ast::ErrorKind::GroupNameDuplicate {
                    original: names[i].span,
                }))
            }
        }
    }
}

#[derive(Clone)]
struct Span {
    // Assume Span has relevant fields
}

mod ast {
    #[derive(Clone)]
    pub struct Error {
        // Assume Error has relevant fields for creation
    }

    #[derive(Clone)]
    pub enum ErrorKind {
        GroupNameDuplicate { original: Span },
    }

    impl Error {
        pub fn new(span: Span, kind: ErrorKind) -> Self {
            // Construct an Error instance
            Error {}
        }
    }
}

#[test]
fn test_add_capture_name_duplicate() {
    let parser = Parser::default();
    let ast_parser = AstParser { parser };

    let cap1 = CaptureName { name: "group1".to_string(), span: Span {} };
    let cap2 = CaptureName { name: "group1".to_string(), span: Span {} };

    // Add the first capture name
    assert!(ast_parser.add_capture_name(&cap1).is_ok());
    
    // Try adding the duplicate capture name
    let result = ast_parser.add_capture_name(&cap2);
    assert!(result.is_err());
}

#[test]
fn test_add_capture_name_success() {
    let parser = Parser::default();
    let ast_parser = AstParser { parser };

    let cap1 = CaptureName { name: "group1".to_string(), span: Span {} };
    let cap2 = CaptureName { name: "group2".to_string(), span: Span {} };

    // Add the first capture name
    assert!(ast_parser.add_capture_name(&cap1).is_ok());
    
    // Add a different capture name
    assert!(ast_parser.add_capture_name(&cap2).is_ok());
}

