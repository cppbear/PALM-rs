// Answer 0

#[test]
fn test_visit_class_set_item_post_empty() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> Result<()> {
            match *ast {
                ast::ClassSetItem::Empty(_)
                | ast::ClassSetItem::Literal(_)
                | ast::ClassSetItem::Range(_)
                | ast::ClassSetItem::Ascii(_)
                | ast::ClassSetItem::Unicode(_)
                | ast::ClassSetItem::Perl(_) => {
                    Ok(())
                }
                ast::ClassSetItem::Bracketed(_)
                | ast::ClassSetItem::Union(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let result = visitor.visit_class_set_item_post(&ast::ClassSetItem::Empty(()));
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_range() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> Result<()> {
            match *ast {
                ast::ClassSetItem::Empty(_)
                | ast::ClassSetItem::Literal(_)
                | ast::ClassSetItem::Range(_)
                | ast::ClassSetItem::Ascii(_)
                | ast::ClassSetItem::Unicode(_)
                | ast::ClassSetItem::Perl(_) => {
                    Ok(())
                }
                ast::ClassSetItem::Bracketed(_)
                | ast::ClassSetItem::Union(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let result = visitor.visit_class_set_item_post(&ast::ClassSetItem::Range(()));
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_literal() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> Result<()> {
            match *ast {
                ast::ClassSetItem::Empty(_)
                | ast::ClassSetItem::Literal(_)
                | ast::ClassSetItem::Range(_)
                | ast::ClassSetItem::Ascii(_)
                | ast::ClassSetItem::Unicode(_)
                | ast::ClassSetItem::Perl(_) => {
                    Ok(())
                }
                ast::ClassSetItem::Bracketed(_)
                | ast::ClassSetItem::Union(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let result = visitor.visit_class_set_item_post(&ast::ClassSetItem::Literal(()));
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_perl() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> Result<()> {
            match *ast {
                ast::ClassSetItem::Empty(_)
                | ast::ClassSetItem::Literal(_)
                | ast::ClassSetItem::Range(_)
                | ast::ClassSetItem::Ascii(_)
                | ast::ClassSetItem::Unicode(_)
                | ast::ClassSetItem::Perl(_) => {
                    Ok(())
                }
                ast::ClassSetItem::Bracketed(_)
                | ast::ClassSetItem::Union(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let result = visitor.visit_class_set_item_post(&ast::ClassSetItem::Perl(()));
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_unicode() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> Result<()> {
            match *ast {
                ast::ClassSetItem::Empty(_)
                | ast::ClassSetItem::Literal(_)
                | ast::ClassSetItem::Range(_)
                | ast::ClassSetItem::Ascii(_)
                | ast::ClassSetItem::Unicode(_)
                | ast::ClassSetItem::Perl(_) => {
                    Ok(())
                }
                ast::ClassSetItem::Bracketed(_)
                | ast::ClassSetItem::Union(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let result = visitor.visit_class_set_item_post(&ast::ClassSetItem::Unicode(()));
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_ascii() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> Result<()> {
            match *ast {
                ast::ClassSetItem::Empty(_)
                | ast::ClassSetItem::Literal(_)
                | ast::ClassSetItem::Range(_)
                | ast::ClassSetItem::Ascii(_)
                | ast::ClassSetItem::Unicode(_)
                | ast::ClassSetItem::Perl(_) => {
                    Ok(())
                }
                ast::ClassSetItem::Bracketed(_)
                | ast::ClassSetItem::Union(_) => {
                    self.decrement_depth();
                    Ok(())
                }
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let result = visitor.visit_class_set_item_post(&ast::ClassSetItem::Ascii(()));
    assert_eq!(result, Ok(()));
}

