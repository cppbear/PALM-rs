// Answer 0

#[test]
fn test_add_item_duplicate_flag() {
    let mut flags = Flags {
        span: Span {
            start: Position { line: 0, column: 0 },
            end: Position { line: 0, column: 10 },
        },
        items: vec![
            FlagsItem {
                span: Span {
                    start: Position { line: 0, column: 0 },
                    end: Position { line: 0, column: 1 },
                },
                kind: FlagsItemKind::Flag(Flag::new("a")),
            },
            FlagsItem {
                span: Span {
                    start: Position { line: 0, column: 1 },
                    end: Position { line: 0, column: 2 },
                },
                kind: FlagsItemKind::Flag(Flag::new("b")),
            },
        ],
    };
    let item = FlagsItem {
        span: Span {
            start: Position { line: 0, column: 2 },
            end: Position { line: 0, column: 3 },
        },
        kind: FlagsItemKind::Flag(Flag::new("a")),
    };
    flags.add_item(item);
}

#[test]
fn test_add_item_duplicate_negation() {
    let mut flags = Flags {
        span: Span {
            start: Position { line: 0, column: 0 },
            end: Position { line: 0, column: 10 },
        },
        items: vec![
            FlagsItem {
                span: Span {
                    start: Position { line: 0, column: 0 },
                    end: Position { line: 0, column: 1 },
                },
                kind: FlagsItemKind::Negation,
            },
        ],
    };
    let item = FlagsItem {
        span: Span {
            start: Position { line: 0, column: 1 },
            end: Position { line: 0, column: 2 },
        },
        kind: FlagsItemKind::Negation,
    };
    flags.add_item(item);
}

#[test]
fn test_add_item_new_flag() {
    let mut flags = Flags {
        span: Span {
            start: Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 10 } },
            end: Span { start: Position { line: 0, column: 10 }, end: Position { line: 0, column: 20 } },
        },
        items: vec![
            FlagsItem {
                span: Span {
                    start: Position { line: 0, column: 0 },
                    end: Position { line: 0, column: 1 },
                },
                kind: FlagsItemKind::Flag(Flag::new("a")),
            },
        ],
    };
    let item = FlagsItem {
        span: Span {
            start: Position { line: 0, column: 1 },
            end: Position { line: 0, column: 3 },
        },
        kind: FlagsItemKind::Flag(Flag::new("b")),
    };
    flags.add_item(item);
}

