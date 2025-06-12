// Answer 0

fn fmt_flags_test() {
    use std::fmt;
    use std::collections::VecDeque;

    struct Writer {
        buffer: VecDeque<char>,
    }

    impl Writer {
        fn new() -> Self {
            Writer {
                buffer: VecDeque::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.extend(s.chars());
            Ok(())
        }

        fn to_string(&self) -> String {
            self.buffer.iter().collect()
        }
    }

    struct Flags {
        items: Vec<FlagsItem>,
    }

    struct FlagsItem {
        kind: FlagsItemKind,
    }

    enum FlagsItemKind {
        Negation,
        Flag(Flag),
    }

    #[derive(Clone)]
    enum Flag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }

    #[test]
    fn test_fmt_flags_with_negation() {
        let ast = Flags {
            items: vec![
                FlagsItem { kind: FlagsItemKind::Negation },
                FlagsItem { kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
            ],
        };
        let mut writer = Writer::new();
        writer.write_str("Result: ").unwrap();
        
        fmt_flags(&mut writer, &ast).unwrap();
        
        assert_eq!(writer.to_string(), "Result: -i");
    }

    #[test]
    fn test_fmt_flags_with_multiple_items() {
        let ast = Flags {
            items: vec![
                FlagsItem { kind: FlagsItemKind::Negation },
                FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
                FlagsItem { kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace) },
            ],
        };
        let mut writer = Writer::new();
        fmt_flags(&mut writer, &ast).unwrap();
        
        assert_eq!(writer.to_string(), "-mx");
    }
}

