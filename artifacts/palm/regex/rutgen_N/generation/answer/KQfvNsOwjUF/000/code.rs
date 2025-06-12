// Answer 0

#[test]
fn test_multi_line_enable() {
    struct MockHir {
        multi_line_enabled: bool,
    }

    impl MockHir {
        fn multi_line(&mut self, yes: bool) {
            self.multi_line_enabled = yes;
        }
    }

    struct ParserBuilder {
        hir: MockHir,
    }

    let mut builder = ParserBuilder {
        hir: MockHir {
            multi_line_enabled: false,
        },
    };

    builder.multi_line(true);
    assert_eq!(builder.hir.multi_line_enabled, true);
}

#[test]
fn test_multi_line_disable() {
    struct MockHir {
        multi_line_enabled: bool,
    }

    impl MockHir {
        fn multi_line(&mut self, yes: bool) {
            self.multi_line_enabled = yes;
        }
    }

    struct ParserBuilder {
        hir: MockHir,
    }

    let mut builder = ParserBuilder {
        hir: MockHir {
            multi_line_enabled: true,
        },
    };

    builder.multi_line(false);
    assert_eq!(builder.hir.multi_line_enabled, false);
}

