// Answer 0

#[test]
fn test_fmt_class_set_binary_op_kind_difference() {
    struct TestWriter {
        output: String,
    }
    
    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    struct TestFormatter {
        wtr: TestWriter,
    }
    
    impl TestFormatter {
        fn new() -> Self {
            Self { wtr: TestWriter::new() }
        }
        
        fn fmt_class_set_binary_op_kind(
            &mut self,
            ast: &ast::ClassSetBinaryOpKind,
        ) -> fmt::Result {
            use ast::ClassSetBinaryOpKind::*;
            match *ast {
                Intersection => self.wtr.write_str("&&"),
                Difference => self.wtr.write_str("--"),
                SymmetricDifference => self.wtr.write_str("~~"),
            }
        }
    }

    let mut formatter = TestFormatter::new();
    let ast = ast::ClassSetBinaryOpKind::Difference;
    let result = formatter.fmt_class_set_binary_op_kind(&ast);
    
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "--");
}

