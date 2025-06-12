// Answer 0

#[test]
fn test_visit_class_set_item_post_literal() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: Writer,
    }

    impl TestVisitor {
        fn fmt_literal(&mut self, _: &str) -> Result<(), std::fmt::Error> {
            self.wtr.write_str("literal")
        }

        fn visit_class_set_item_post(
            &mut self,
            ast: &ast::ClassSetItem,
        ) -> Result<(), std::fmt::Error> {
            use ast::ClassSetItem::*;

            match *ast {
                Empty(_) => Ok(()),
                Literal(ref x) => self.fmt_literal(x),
                Range(ref x) => {
                    self.fmt_literal(&x.start)?;
                    self.wtr.write_str("-")?;
                    self.fmt_literal(&x.end)?;
                    Ok(())
                }
                Ascii(ref x) => self.fmt_class_ascii(x),
                Unicode(ref x) => self.fmt_class_unicode(x),
                Perl(ref x) => self.fmt_class_perl(x),
                Bracketed(ref x) => self.fmt_class_bracketed_post(x),
                Union(_) => Ok(()),
            }
        }
    }

    mod ast {
        pub struct ClassSetItem;

        impl ClassSetItem {
            pub fn Literal(x: &str) -> ClassSetItem {
                ClassSetItem
            }
        }
    }

    let mut visitor = TestVisitor {
        wtr: Writer { output: String::new() },
    };
    
    let literal_item = ast::ClassSetItem::Literal("test");
    
    let result = visitor.visit_class_set_item_post(&literal_item);
    
    assert!(result.is_ok());
    assert_eq!(visitor.wtr.output, "literal");
}

