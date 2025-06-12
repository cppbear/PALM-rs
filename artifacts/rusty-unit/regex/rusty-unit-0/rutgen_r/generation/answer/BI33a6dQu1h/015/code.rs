// Answer 0

#[test]
fn test_fmt_flags_empty() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter<'a> {
        wtr: &'a mut Writer,
    }

    impl<'a> Formatter<'a> {
        fn fmt_flags(&mut self, ast: &Flags) -> std::fmt::Result {
            use ast::{Flag, FlagsItemKind};

            for item in &ast.items {
                match item.kind {
                    FlagsItemKind::Negation => self.wtr.write_str("-"),
                    FlagsItemKind::Flag(ref flag) => {
                        match *flag {
                            Flag::CaseInsensitive => self.wtr.write_str("i"),
                            Flag::MultiLine => self.wtr.write_str("m"),
                            Flag::DotMatchesNewLine => self.wtr.write_str("s"),
                            Flag::SwapGreed => self.wtr.write_str("U"),
                            Flag::Unicode => self.wtr.write_str("u"),
                            Flag::IgnoreWhitespace => self.wtr.write_str("x"),
                        }
                    }
                }?;
            }
            Ok(())
        }
    }

    struct Flags {
        items: Vec<FlagsItem>,
    }

    struct FlagsItem {
        kind: FlagsItemKind,
    }

    mod ast {
        pub enum Flag {
            CaseInsensitive,
            MultiLine,
            DotMatchesNewLine,
            SwapGreed,
            Unicode,
            IgnoreWhitespace,
        }

        pub enum FlagsItemKind {
            Negation,
            Flag(Flag),
        }
    }

    let ast = Flags { items: vec![] }; // empty items to satisfy constraint of item in &ast.items is false
    let mut writer = Writer::new();
    let mut formatter = Formatter { wtr: &mut writer };

    let result = formatter.fmt_flags(&ast);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, "");
}

