// Answer 0

#[test]
fn test_visit_class_set_item_post_perl() {
    struct DummyVisitor {
        depth: usize,
    }

    impl DummyVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(
            &mut self,
            ast: &ast::ClassSetItem,
        ) -> Result<()> {
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

    let mut visitor = DummyVisitor { depth: 5 };
    let ast = ast::ClassSetItem::Perl(ast::Perl::new("some_perl_pattern"));
    
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

#[test]
fn test_visit_class_set_item_post_empty() {
    struct DummyVisitor {
        depth: usize,
    }

    impl DummyVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(
            &mut self,
            ast: &ast::ClassSetItem,
        ) -> Result<()> {
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

    let mut visitor = DummyVisitor { depth: 5 };
    let ast = ast::ClassSetItem::Empty(ast::Empty::new());
    
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

#[test]
fn test_visit_class_set_item_post_range() {
    struct DummyVisitor {
        depth: usize,
    }

    impl DummyVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(
            &mut self,
            ast: &ast::ClassSetItem,
        ) -> Result<()> {
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

    let mut visitor = DummyVisitor { depth: 5 };
    let ast = ast::ClassSetItem::Range(ast::Range::new('a', 'z'));
    
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

#[test]
fn test_visit_class_set_item_post_literal() {
    struct DummyVisitor {
        depth: usize,
    }

    impl DummyVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(
            &mut self,
            ast: &ast::ClassSetItem,
        ) -> Result<()> {
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

    let mut visitor = DummyVisitor { depth: 5 };
    let ast = ast::ClassSetItem::Literal(ast::Literal::new('x'));
    
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

#[test]
fn test_visit_class_set_item_post_ascii() {
    struct DummyVisitor {
        depth: usize,
    }

    impl DummyVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(
            &mut self,
            ast: &ast::ClassSetItem,
        ) -> Result<()> {
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

    let mut visitor = DummyVisitor { depth: 5 };
    let ast = ast::ClassSetItem::Ascii(ast::Ascii::new('c'));
    
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

#[test]
fn test_visit_class_set_item_post_unicode() {
    struct DummyVisitor {
        depth: usize,
    }

    impl DummyVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
        
        fn visit_class_set_item_post(
            &mut self,
            ast: &ast::ClassSetItem,
        ) -> Result<()> {
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

    let mut visitor = DummyVisitor { depth: 5 };
    let ast = ast::ClassSetItem::Unicode(ast::Unicode::new('δ'));
    
    assert_eq!(visitor.visit_class_set_item_post(&ast), Ok(()));
}

