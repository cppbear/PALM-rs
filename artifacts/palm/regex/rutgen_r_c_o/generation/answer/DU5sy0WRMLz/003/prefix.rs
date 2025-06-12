// Answer 0

#[test]
fn test_add_item_empty() {
    let mut flags = Flags {
        span: Span { start: Position(0), end: Position(1) },
        items: Vec::new(),
    };
    let item = FlagsItem {
        span: Span { start: Position(0), end: Position(1) },
        kind: FlagsItemKind::Negation,
    };
    flags.add_item(item);
}

#[test]
fn test_add_item_unique_negation() {
    let mut flags = Flags {
        span: Span { start: Position(0), end: Position(1) },
        items: vec![
            FlagsItem {
                span: Span { start: Position(0), end: Position(1) },
                kind: FlagsItemKind::Flag(Flag::A),
            },
        ],
    };
    let item = FlagsItem {
        span: Span { start: Position(2), end: Position(3) },
        kind: FlagsItemKind::Negation,
    };
    flags.add_item(item);
}

#[test]
fn test_add_item_unique_flag() {
    let mut flags = Flags {
        span: Span { start: Position(0), end: Position(1) },
        items: vec![
            FlagsItem {
                span: Span { start: Position(0), end: Position(1) },
                kind: FlagsItemKind::Negation,
            },
        ],
    };
    let item = FlagsItem {
        span: Span { start: Position(2), end: Position(3) },
        kind: FlagsItemKind::Flag(Flag::B),
    };
    flags.add_item(item);
}

#[test]
fn test_add_item_max_items() {
    let mut flags = Flags {
        span: Span { start: Position(0), end: Position(1) },
        items: vec![
            FlagsItem {
                span: Span { start: Position(0), end: Position(1) },
                kind: FlagsItemKind::Flag(Flag::A),
            },
            FlagsItem {
                span: Span { start: Position(1), end: Position(2) },
                kind: FlagsItemKind::Flag(Flag::B),
            },
            FlagsItem {
                span: Span { start: Position(2), end: Position(3) },
                kind: FlagsItemKind::Flag(Flag::C),
            },
            FlagsItem {
                span: Span { start: Position(3), end: Position(4) },
                kind: FlagsItemKind::Flag(Flag::D),
            },
            FlagsItem {
                span: Span { start: Position(4), end: Position(5) },
                kind: FlagsItemKind::Flag(Flag::E),
            },
            FlagsItem {
                span: Span { start: Position(5), end: Position(6) },
                kind: FlagsItemKind::Flag(Flag::F),
            },
            FlagsItem {
                span: Span { start: Position(6), end: Position(7) },
                kind: FlagsItemKind::Flag(Flag::G),
            },
            FlagsItem {
                span: Span { start: Position(7), end: Position(8) },
                kind: FlagsItemKind::Flag(Flag::H),
            },
            FlagsItem {
                span: Span { start: Position(8), end: Position(9) },
                kind: FlagsItemKind::Flag(Flag::I),
            },
            FlagsItem {
                span: Span { start: Position(9), end: Position(10) },
                kind: FlagsItemKind::Flag(Flag::J),
            },
        ],
    };
    let item = FlagsItem {
        span: Span { start: Position(10), end: Position(11) },
        kind: FlagsItemKind::Negation,
    };
    flags.add_item(item);
}

