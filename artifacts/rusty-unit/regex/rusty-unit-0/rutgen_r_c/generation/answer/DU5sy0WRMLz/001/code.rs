// Answer 0

#[test]
fn test_add_item_duplicate() {
    let mut flags = Flags {
        span: Span { start: Position(0), end: Position(10) },
        items: vec![
            FlagsItem {
                span: Span { start: Position(0), end: Position(5) },
                kind: FlagsItemKind::Flag(Flag::SomeFlag),
            },
        ],
    };

    let new_item = FlagsItem {
        span: Span { start: Position(5), end: Position(10) },
        kind: FlagsItemKind::Flag(Flag::SomeFlag),
    };

    let result = flags.add_item(new_item);
    assert_eq!(result, Some(0)); // The index of the duplicate item
}

#[test]
fn test_add_item_success() {
    let mut flags = Flags {
        span: Span { start: Position(0), end: Position(10) },
        items: vec![
            FlagsItem {
                span: Span { start: Position(0), end: Position(5) },
                kind: FlagsItemKind::Flag(Flag::SomeFlag),
            },
        ],
    };

    let new_item = FlagsItem {
        span: Span { start: Position(5), end: Position(10) },
        kind: FlagsItemKind::Flag(Flag::AnotherFlag),
    };

    let result = flags.add_item(new_item);
    assert_eq!(result, None); // The item was added successfully
}

#[test]
fn test_add_item_ignore_negation() {
    let mut flags = Flags {
        span: Span { start: Position(0), end: Position(10) },
        items: vec![
            FlagsItem {
                span: Span { start: Position(0), end: Position(5) },
                kind: FlagsItemKind::Negation,
            },
        ],
    };

    let new_item = FlagsItem {
        span: Span { start: Position(5), end: Position(10) },
        kind: FlagsItemKind::Negation,
    };

    let result = flags.add_item(new_item);
    assert_eq!(result, Some(0)); // The index of the duplicate negation
}

