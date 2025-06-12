// Answer 0

#[test]
fn test_add_capture_name_new_name_success() {
    struct Parser {
        capture_names: std::cell::RefCell<Vec<ast::CaptureName>>,
    }

    impl Parser {
        fn capture_names(&self) -> std::cell::RefCell<Vec<ast::CaptureName>> {
            self.capture_names.clone()
        }

        fn error(&self, _span: ast::Span, _kind: ast::ErrorKind) -> ast::Error {
            // Placeholder implementation
            ast::Error::new() 
        }
    }

    impl Parser {
        fn add_capture_name(&self, cap: &ast::CaptureName) -> Result<()> {
            let mut names = self.capture_names.borrow_mut();
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

    let parser = Parser {
        capture_names: std::cell::RefCell::new(vec![]),
    };

    let cap = ast::CaptureName {
        name: "new_capture".to_string(),
        span: ast::Span::new(0, 10),
    };

    assert_eq!(parser.add_capture_name(&cap), Ok(()));
}

#[test]
fn test_add_capture_name_duplicate_name_failure() {
    struct Parser {
        capture_names: std::cell::RefCell<Vec<ast::CaptureName>>,
    }

    impl Parser {
        fn capture_names(&self) -> std::cell::RefCell<Vec<ast::CaptureName>> {
            self.capture_names.clone()
        }

        fn error(&self, _span: ast::Span, _kind: ast::ErrorKind) -> ast::Error {
            // Placeholder implementation
            ast::Error::new() 
        }
    }

    impl Parser {
        fn add_capture_name(&self, cap: &ast::CaptureName) -> Result<()> {
            let mut names = self.capture_names.borrow_mut();
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

    let parser = Parser {
        capture_names: std::cell::RefCell::new(vec![
            ast::CaptureName {
                name: "duplicate".to_string(),
                span: ast::Span::new(0, 10),
            },
        ]),
    };

    let cap = ast::CaptureName {
        name: "duplicate".to_string(),
        span: ast::Span::new(10, 20),
    };

    assert!(parser.add_capture_name(&cap).is_err());
}

