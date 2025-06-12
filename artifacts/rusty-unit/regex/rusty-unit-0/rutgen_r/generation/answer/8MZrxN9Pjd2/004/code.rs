// Answer 0

#[test]
fn test_visit_pre_non_capturing_group() {
    use regex_syntax::hir::{Hir, HirKind, Group, GroupKind};

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

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { wtr: TestWriter::new() }
        }

        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            match *hir.kind() {
                HirKind::Group(ref x) => {
                    match x.kind {
                        GroupKind::NonCapturing => {
                            self.wtr.write_str("(?:")?;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            Ok(())
        }
    }

    let mut visitor = TestVisitor::new();
    let hir = Hir::from_kind(HirKind::Group(Group { kind: GroupKind::NonCapturing }));

    assert_eq!(visitor.visit_pre(&hir), Ok(()));
    assert_eq!(visitor.wtr.output, "(?:");
}

