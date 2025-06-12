// Answer 0

#[test]
fn test_visit_pre_group_capture_index() {
    struct MockWriter {
        result: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.result.push_str(s);
            Ok(())
        }
    }
    
    let mut mock_writer = MockWriter { result: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let group_kind = hir::GroupKind::CaptureIndex(1);
    let group_hir = Hir {
        kind: HirKind::Group(Group { kind: group_kind, hir: Box::new(Hir::empty()) }),
        info: HirInfo::default(),
    };

    let result = writer.visit_pre(&group_hir);
    assert_eq!(result, Ok(()));
    assert_eq!(mock_writer.result, "(");
}

#[test]
fn test_visit_pre_group_capture_name() {
    struct MockWriter {
        result: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.result.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { result: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let group_kind = hir::GroupKind::CaptureName { name: String::from("group1"), index: 0 };
    let group_hir = Hir {
        kind: HirKind::Group(Group { kind: group_kind, hir: Box::new(Hir::empty()) }),
        info: HirInfo::default(),
    };

    let result = writer.visit_pre(&group_hir);
    assert_eq!(result, Ok(()));
    assert_eq!(mock_writer.result, "(?P<group1>");
}

#[test]
fn test_visit_pre_group_non_capturing() {
    struct MockWriter {
        result: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.result.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { result: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let group_kind = hir::GroupKind::NonCapturing;
    let group_hir = Hir {
        kind: HirKind::Group(Group { kind: group_kind, hir: Box::new(Hir::empty()) }),
        info: HirInfo::default(),
    };

    let result = writer.visit_pre(&group_hir);
    assert_eq!(result, Ok(()));
    assert_eq!(mock_writer.result, "(?:");
}

