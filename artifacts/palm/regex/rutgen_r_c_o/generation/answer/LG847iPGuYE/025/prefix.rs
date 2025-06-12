// Answer 0

#[test]
fn test_visit_post_zero_or_one_write_err() {
    struct TestWriter {
        should_fail: bool,
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                self.output.push_str(s);
                Ok(())
            }
        }
    }

    let mut writer = TestWriter { should_fail: true, output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let repetition = Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(Hir::empty()),
    };
    
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        info: HirInfo::default(),
    };

    let _ = visitor.visit_post(&hir);
}

