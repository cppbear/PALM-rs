// Answer 0

#[test]
fn test_visit_pre_group_capture_name() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            Self {
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
            Self {
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

    #[derive(Clone)]
    enum HirKind {
        Group(Group),
    }

    struct Group {
        kind: GroupKind,
    }

    enum GroupKind {
        CaptureName { name: String },
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut visitor = MockVisitor::new();
    let hir = Hir {
        kind: HirKind::Group(Group {
            kind: GroupKind::CaptureName {
                name: "test_group".to_string(),
            },
        }),
    };

    let result = visitor.visit_pre(&hir);

    assert_eq!(result, Ok(()));
    assert_eq!(visitor.wtr.output, "(?P<test_group>");
}

