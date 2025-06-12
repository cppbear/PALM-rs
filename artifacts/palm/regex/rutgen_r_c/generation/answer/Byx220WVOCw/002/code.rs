// Answer 0

#[test]
fn test_add_capture_name_duplicate() {
    use ast::{CaptureName, ErrorKind, Span};
    use std::cell::RefCell;
    
    struct MockParser {
        capture_names: RefCell<Vec<CaptureName>>,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                capture_names: RefCell::new(vec![]),
            }
        }
        
        fn error(&self, span: Span, kind: ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }
        
        fn add_capture_name(&self, cap: &CaptureName) -> Result<()> {
            let mut names = self.capture_names.borrow_mut();
            match names.binary_search_by_key(&cap.name.as_str(), |c| c.name.as_str()) {
                Err(i) => {
                    names.insert(i, cap.clone());
                    Ok(())
                }
                Ok(i) => {
                    Err(self.error(cap.span, ErrorKind::GroupNameDuplicate {
                        original: names[i].span,
                    }))
                }
            }
        }
    }

    let parser = MockParser::new();
    let span = Span { start: 0, end: 1 };
    
    let cap1 = CaptureName { span, name: "duplicate".to_string(), index: 1 };
    let cap2 = CaptureName { span, name: "duplicate".to_string(), index: 2 };

    // Add the first capture name, expecting it to succeed
    assert!(parser.add_capture_name(&cap1).is_ok());
    
    // Add the second capture name with the same name, expecting an error
    let result = parser.add_capture_name(&cap2);
    assert!(result.is_err());

    if let Err(error) = result {
        assert_eq!(error.kind, ErrorKind::GroupNameDuplicate { original: cap1.span });
    }
}

