// Answer 0

#[test]
fn test_visit_class_set_item_post_empty() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth = self.depth.saturating_sub(1);
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
    let ast = ast::ClassSetItem::Empty(ast::Empty);
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

#[test]
fn test_visit_class_set_item_post_literal() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth = self.depth.saturating_sub(1);
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
    let ast = ast::ClassSetItem::Literal(ast::Literal::new('a'));
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

#[test]
fn test_visit_class_set_item_post_range() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth = self.depth.saturating_sub(1);
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
    let ast = ast::ClassSetItem::Range(ast::Range::new('a', 'z'));
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

#[test]
fn test_visit_class_set_item_post_ascii() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth = self.depth.saturating_sub(1);
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
    let ast = ast::ClassSetItem::Ascii('a');
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

#[test]
fn test_visit_class_set_item_post_unicode() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth = self.depth.saturating_sub(1);
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
    let ast = ast::ClassSetItem::Unicode('汉');
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

#[test]
fn test_visit_class_set_item_post_perl() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth = self.depth.saturating_sub(1);
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
    let ast = ast::ClassSetItem::Perl(ast::Perl);
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

