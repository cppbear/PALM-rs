// Answer 0

#[test]
fn test_fmt_class_bracketed_pre_not_negated() {
    use std::fmt::Write;

    struct TestVisitor {
        output: String,
    }

    impl Visitor for TestVisitor {
        type Output = String;
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let mut printer = Printer { _priv: () };
    let mut test_writer = TestVisitor { output: String::new() };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut test_writer,
    };
    
    let class_bracketed = ClassBracketed {
        span: Span { /* initialize if necessary */ },
        negated: false,
        kind: ClassSet { /* initialize if necessary */ },
    };
    
    let result = writer.fmt_class_bracketed_pre(&class_bracketed);
    
    assert!(result.is_ok());
    assert_eq!(test_writer.output, "[");

}

#[test]
fn test_fmt_class_bracketed_pre_negated() {
    use std::fmt::Write;

    struct TestVisitor {
        output: String,
    }

    impl Visitor for TestVisitor {
        type Output = String;
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let mut printer = Printer { _priv: () };
    let mut test_writer = TestVisitor { output: String::new() };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut test_writer,
    };
    
    let class_bracketed = ClassBracketed {
        span: Span { /* initialize if necessary */ },
        negated: true,
        kind: ClassSet { /* initialize if necessary */ },
    };
    
    let result = writer.fmt_class_bracketed_pre(&class_bracketed);
    
    assert!(result.is_ok());
    assert_eq!(test_writer.output, "[^");
}

