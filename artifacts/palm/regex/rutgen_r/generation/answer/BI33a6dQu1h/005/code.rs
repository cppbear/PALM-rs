// Answer 0

fn test_fmt_flags() {
    use std::fmt;
    use std::fmt::Write;
    
    struct Writer {
        output: String,
    }
    
    impl Writer {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }
    
    impl fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    struct AstFlags {
        items: Vec<AstItem>,
    }
    
    struct AstItem {
        kind: FlagsItemKind,
    }
    
    enum FlagsItemKind {
        Negation,
        Flag(Flag),
    }
    
    enum Flag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }
    
    let mut writer = Writer::new();
    
    // Valid case: testing Flag::SwapGreed
    {
        let ast = AstFlags {
            items: vec![
                AstItem { kind: FlagsItemKind::Flag(Flag::SwapGreed) },
            ],
        };
        let result = writer.write_str(&ast);
        assert!(result.is_ok());
        assert_eq!(writer.output, "U");
    }

    // Valid case: testing multiple flags including Flag::SwapGreed
    {
        writer = Writer::new(); // Reset writer

        let ast = AstFlags {
            items: vec![
                AstItem { kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
                AstItem { kind: FlagsItemKind::Flag(Flag::SwapGreed) },
                AstItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
            ],
        };
        let result = writer.write_str(&ast);
        assert!(result.is_ok());
        assert_eq!(writer.output, "iUm");
    }

    // Valid case: testing Negation
    {
        writer = Writer::new(); // Reset writer

        let ast = AstFlags {
            items: vec![
                AstItem { kind: FlagsItemKind::Negation },
                AstItem { kind: FlagsItemKind::Flag(Flag::Unicode) },
            ],
        };
        let result = writer.write_str(&ast);
        assert!(result.is_ok());
        assert_eq!(writer.output, "-u");
    }

    // Empty flags, should not panic
    {
        writer = Writer::new(); // Reset writer

        let ast = AstFlags { items: Vec::new() };
        let result = writer.write_str(&ast);
        assert!(result.is_ok());
        assert_eq!(writer.output, "");
    }

    // Boundary condition: testing the limit of Flag variations
    {
        writer = Writer::new(); // Reset writer

        let ast = AstFlags {
            items: vec![
                AstItem { kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
                AstItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
                AstItem { kind: FlagsItemKind::Flag(Flag::DotMatchesNewLine) },
                AstItem { kind: FlagsItemKind::Flag(Flag::SwapGreed) },
                AstItem { kind: FlagsItemKind::Flag(Flag::Unicode) },
                AstItem { kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace) },
            ],
        };
        let result = writer.write_str(&ast);
        assert!(result.is_ok());
        assert_eq!(writer.output, "imsUux");
    }
}

