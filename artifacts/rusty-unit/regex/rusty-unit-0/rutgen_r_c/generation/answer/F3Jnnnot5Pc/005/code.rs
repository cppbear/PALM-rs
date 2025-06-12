// Answer 0

#[test]
fn test_fmt_group_pre_capture_name_success() {
    use std::fmt::Write as FmtWrite;

    struct MockWriter {
        output: String,
    }

    impl FmtWrite for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: mock_writer,
    };

    let capture_name = ast::CaptureName {
        span: Span::default(),
        name: String::from("my_capture"),
        index: 0,
    };

    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::CaptureName(capture_name),
        ast: Box::new(Ast::default()),
    };

    let result = writer.fmt_group_pre(&group);
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "(?P<my_capture>");
}

#[test]
#[should_panic(expected = "Err")]
fn test_fmt_group_pre_capture_name_write_str_err() {
    struct MockErrWriter;

    impl fmt::Write for MockErrWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut mock_writer = MockErrWriter;
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: mock_writer,
    };

    let capture_name = ast::CaptureName {
        span: Span::default(),
        name: String::from("my_capture"),
        index: 0,
    };

    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::CaptureName(capture_name),
        ast: Box::new(Ast::default()),
    };

    let _ = writer.fmt_group_pre(&group);
}

