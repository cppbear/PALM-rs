// Answer 0

#[test]
fn test_visit_pre_group_capture_index() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockVisitor {
        wtr: MockWriter,
    }

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor {
                wtr: MockWriter::new(),
            }
        }

        fn write_literal_char(&mut self, _c: char) -> fmt::Result {
            Ok(())
        }

        fn write_literal_byte(&mut self, _b: u8) -> fmt::Result {
            Ok(())
        }

        fn write_literal_class_byte(&mut self, _b: u8) -> fmt::Result {
            Ok(())
        }
    }

    struct MockHir {
        kind: HirKind,
    }

    enum HirKind {
        Group(MockGroup),
    }

    struct MockGroup {
        kind: hir::GroupKind,
    }

    mod hir {
        pub struct GroupKind;
        pub struct GroupKindCaptureIndex(pub usize);
    }

    let mut visitor = MockVisitor::new();
    let group = MockGroup { kind: hir::GroupKindCaptureIndex(1) };
    let hir = MockHir { kind: HirKind::Group(group) };

    let result = visitor.visit_pre(&hir);

    assert_eq!(result, Ok(()));
    assert_eq!(visitor.wtr.output, "(");
}

