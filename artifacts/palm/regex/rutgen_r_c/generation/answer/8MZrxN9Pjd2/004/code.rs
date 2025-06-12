// Answer 0

#[test]
fn test_visit_pre_group_non_capturing() {
    use hir::{Hir, HirKind, Group, GroupKind};
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let group = Group {
        kind: GroupKind::NonCapturing,
        hir: Box::new(Hir { kind: HirKind::Empty, info: Default::default() }),
    };
    let hir = Hir { kind: HirKind::Group(group), info: Default::default() };

    let result = visitor.visit_pre(&hir);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "(?:");
}

