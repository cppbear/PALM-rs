// Answer 0

#[test]
fn test_fmt_group_pre_capture_name() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter<W> {
        wtr: W,
    }

    impl<W: Write> Formatter<W> {
        fn fmt_group_pre(&mut self, ast: &ast::Group) -> fmt::Result {
            use ast::GroupKind::*;
            match ast.kind {
                CaptureIndex(_) => self.wtr.write_str("("),
                CaptureName(ref x) => {
                    self.wtr.write_str("(?P<")?;
                    self.wtr.write_str(&x.name)?;
                    self.wtr.write_str(">")?;
                    Ok(())
                }
                NonCapturing(ref flags) => {
                    self.wtr.write_str("(?")?;
                    self.fmt_flags(flags)?;
                    self.wtr.write_str(":")?;
                    Ok(())
                }
            }
        }
    }

    struct GroupName {
        name: String,
    }

    struct Group {
        kind: ast::GroupKind,
    }

    let mut writer = MockWriter::new();
    let mut formatter = Formatter { wtr: writer };

    let ast = Group {
        kind: ast::GroupKind::CaptureName(GroupName { name: String::from("test_name") }),
    };

    let result = formatter.fmt_group_pre(&ast);

    assert_eq!(result, Ok(()));
    assert_eq!(formatter.wtr.output, "(?P<test_name>)");
}

