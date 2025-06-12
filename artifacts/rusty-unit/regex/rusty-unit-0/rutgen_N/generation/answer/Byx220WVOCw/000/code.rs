// Answer 0

#[test]
fn test_add_capture_name_success() {
    struct MockParser {
        capture_names: std::cell::RefCell<Vec<ast::CaptureName>>,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser {
                capture_names: std::cell::RefCell::new(Vec::new()),
            }
        }

        fn parser(&self) -> &MockParser {
            self
        }

        fn borrow_mut_capture_names(&self) -> std::cell::RefMut<Vec<ast::CaptureName>> {
            self.capture_names.borrow_mut()
        }
    }

    impl MockParser {
        fn add_capture_name(&self, cap: &ast::CaptureName) -> Result<()> {
            let mut names = self.borrow_mut_capture_names();
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

    let parser = MockParser::new();
    let capture_name_1 = ast::CaptureName::new("name1", /* span */ 0..1);
    let result_1 = parser.add_capture_name(&capture_name_1);
    assert!(result_1.is_ok());

    let capture_name_2 = ast::CaptureName::new("name2", /* span */ 2..3);
    let result_2 = parser.add_capture_name(&capture_name_2);
    assert!(result_2.is_ok());
}

#[test]
fn test_add_capture_name_duplicate() {
    struct MockParser {
        capture_names: std::cell::RefCell<Vec<ast::CaptureName>>,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser {
                capture_names: std::cell::RefCell::new(Vec::new()),
            }
        }

        fn parser(&self) -> &MockParser {
            self
        }

        fn borrow_mut_capture_names(&self) -> std::cell::RefMut<Vec<ast::CaptureName>> {
            self.capture_names.borrow_mut()
        }
    }

    impl MockParser {
        fn add_capture_name(&self, cap: &ast::CaptureName) -> Result<()> {
            let mut names = self.borrow_mut_capture_names();
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

    let parser = MockParser::new();
    let capture_name = ast::CaptureName::new("duplicate_name", /* span */ 0..1);
    let _ = parser.add_capture_name(&capture_name);
    let result = parser.add_capture_name(&capture_name);
    assert!(result.is_err());
}

