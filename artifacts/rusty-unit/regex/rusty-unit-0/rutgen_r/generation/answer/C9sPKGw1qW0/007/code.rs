// Answer 0

#[test]
#[should_panic]
fn test_visit_class_set_item_post_range_with_err() {
    use regex_syntax::ast::{self, ClassSetItem, Range};
    
    struct TestWriter {
        output: String,
    }
    
    impl TestWriter {
        fn new() -> Self {
            Self {
                output: String::new(),
            }
        }
        
        fn write_str(&mut self, s: &str) -> Result<(), &'static str> {
            if s == "-" {
                Err("write_str error")
            } else {
                self.output.push_str(s);
                Ok(())
            }
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }
    
    impl TestVisitor {
        fn fmt_literal(&mut self, _val: &str) -> Result<(), &'static str> {
            Ok(())
        }
        
        fn visit_class_set_item_post(
            &mut self,
            ast: &ClassSetItem,
        ) -> Result<(), &'static str> {
            match *ast {
                ClassSetItem::Range(ref x) => {
                    self.fmt_literal(&x.start)?;
                    self.wtr.write_str("-")?;
                    self.fmt_literal(&x.end)?;
                    Ok(())
                },
                _ => Ok(()),
            }
        }
    }

    let mut visitor = TestVisitor {
        wtr: TestWriter::new(),
    };

    let range_item = Range { start: "a".to_string(), end: "z".to_string() };
    let class_set_item = ClassSetItem::Range(range_item);

    let _ = visitor.visit_class_set_item_post(&class_set_item);
}

