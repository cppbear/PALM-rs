// Answer 0

#[test]
fn test_fmt_group_pre_non_capturing_ok() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_ref = Writer { printer: &mut printer, wtr: &mut writer };

    let flags_item = ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) };
    let flags = ast::Flags { span: Span::default(), items: vec![flags_item] };

    let ast = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::NonCapturing(Box::new(flags)),
        ast: Box::new(ast::Ast::default()),
    };

    writer_ref.fmt_group_pre(&ast);
}

#[test]
#[should_panic]
fn test_fmt_group_pre_non_capturing_err() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_ref = Writer { printer: &mut printer, wtr: &mut writer };

    let flags_item = ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) };
    let flags = ast::Flags { span: Span::default(), items: vec![flags_item] };

    let ast = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::NonCapturing(Box::new(flags)),
        ast: Box::new(ast::Ast::default()),
    };

    writer_ref.fmt_group_pre(&ast);
}

#[test]
fn test_fmt_group_pre_non_capturing_with_err() {
    struct MockWriter {
        output: String,
        write_errors: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.write_errors && s == ":" {
                Err(fmt::Error)
            } else {
                self.output.push_str(s);
                Ok(())
            }
        }
    }

    let mut writer = MockWriter { output: String::new(), write_errors: true };
    let mut printer = Printer { _priv: () };
    let mut writer_ref = Writer { printer: &mut printer, wtr: &mut writer };

    let flags_item = ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) };
    let flags = ast::Flags { span: Span::default(), items: vec![flags_item] };

    let ast = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::NonCapturing(Box::new(flags)),
        ast: Box::new(ast::Ast::default()),
    };

    writer_ref.fmt_group_pre(&ast);
}

