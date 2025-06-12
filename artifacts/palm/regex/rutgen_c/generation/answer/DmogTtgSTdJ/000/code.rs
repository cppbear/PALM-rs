// Answer 0

#[test]
fn test_finish_returns_ok() {
    struct TestVisitor<'a> {
        writer: Writer<'a, String>,
    }

    impl<'a> Visitor for TestVisitor<'a> {
        type Output = ();
        type Err = fmt::Error;

        fn finish(self) -> fmt::Result {
            self.writer.finish()
        }

        fn start(&mut self) {}
        fn visit_pre(&mut self, _hir: &Hir) -> fmt::Result {
            Ok(())
        }
        fn visit_post(&mut self, _hir: &Hir) -> fmt::Result {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> fmt::Result {
            Ok(())
        }
    }

    let printer = &mut Printer { _priv: () };
    let writer = Writer {
        printer,
        wtr: String::new(),
    };
    
    let visitor = TestVisitor { writer };
    assert!(visitor.finish().is_ok());
}

#[test]
fn test_finish_is_successful() {
    struct SuccessfulVisitor<'a> {
        writer: Writer<'a, String>,
    }

    impl<'a> Visitor for SuccessfulVisitor<'a> {
        type Output = ();
        type Err = fmt::Error;

        fn finish(self) -> fmt::Result {
            self.writer.finish()
        }

        fn start(&mut self) {}
        fn visit_pre(&mut self, _hir: &Hir) -> fmt::Result {
            Ok(())
        }
        fn visit_post(&mut self, _hir: &Hir) -> fmt::Result {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> fmt::Result {
            Ok(())
        }
    }

    let printer = &mut Printer { _priv: () };
    let writer = Writer {
        printer,
        wtr: String::new(),
    };

    let visitor = SuccessfulVisitor { writer };
    assert!(visitor.finish().is_ok());
}

