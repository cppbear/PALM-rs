// Answer 0

#[test]
fn test_visit_pre_non_capturing_group_write_str_err() {
    use std::fmt::Write;
    use hir::{Hir, HirKind, Group, GroupKind};
    use std::io::Error;

    struct MockWriter {
        buffer: String,
        error: bool,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.error {
                Err(Error)
            } else {
                self.buffer.push_str(s);
                Ok(())
            }
        }
    }

    let mut mock_writer = MockWriter {
        buffer: String::new(),
        error: true,
    };

    let inner_hir = Box::new(Hir {
        kind: HirKind::Empty, // doesn't need to be specific for the mock
        info: Default::default(),
    });

    let group = Group {
        kind: GroupKind::NonCapturing,
        hir: inner_hir,
    };

    let hir = Hir {
        kind: HirKind::Group(group),
        info: Default::default(),
    };

    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: mock_writer,
    };

    let result = writer.visit_pre(&hir);

    assert!(result.is_err());
}

