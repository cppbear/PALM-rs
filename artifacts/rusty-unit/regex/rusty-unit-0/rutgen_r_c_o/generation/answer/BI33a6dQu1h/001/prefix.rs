// Answer 0

#[test]
fn test_fmt_flags_single_ignore_whitespace() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    
    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace),
            },
        ],
    };
    
    writer.fmt_flags(&flags).unwrap();
    // Output: "x"
}

#[test]
fn test_fmt_flags_multiple_ignore_whitespace() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    
    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace),
            },
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace),
            },
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace),
            },
        ],
    };
    
    writer.fmt_flags(&flags).unwrap();
    // Output: "xxx"
}

#[test]
fn test_fmt_flags_with_negation_and_ignore_whitespace() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    
    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Negation,
            },
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace),
            },
        ],
    };
    
    writer.fmt_flags(&flags).unwrap();
    // Output: "-x"
}

#[test]
fn test_fmt_flags_all_flags_with_ignore_whitespace() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    
    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::CaseInsensitive),
            },
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::MultiLine),
            },
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::DotMatchesNewLine),
            },
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::SwapGreed),
            },
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::Unicode),
            },
            FlagsItem {
                span: Span::default(),
                kind: FlagsItemKind::Flag(Flag::IgnoreWhitespace),
            },
        ],
    };
    
    writer.fmt_flags(&flags).unwrap();
    // Output: "iumsx"
}

#[test]
fn test_fmt_flags_empty() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    
    let flags = Flags {
        span: Span::default(),
        items: vec![],
    };
    
    writer.fmt_flags(&flags).unwrap();
    // Output: ""
}

