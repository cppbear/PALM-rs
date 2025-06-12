// Answer 0

#[test]
fn test_add_item_empty() {
    let mut flags = Flags { span: Span { start: Position(0), end: Position(0) }, items: Vec::new() };
    let item = FlagsItem { span: Span { start: Position(0), end: Position(1) }, kind: FlagsItemKind::Flag(Flag::A) };
    flags.add_item(item);
}

#[test]
fn test_add_item_unique() {
    let mut flags = Flags { span: Span { start: Position(0), end: Position(5) }, items: vec![] };
    let item1 = FlagsItem { span: Span { start: Position(0), end: Position(1) }, kind: FlagsItemKind::Flag(Flag::A) };
    let item2 = FlagsItem { span: Span { start: Position(1), end: Position(2) }, kind: FlagsItemKind::Flag(Flag::B) };
    flags.add_item(item1);
    flags.add_item(item2);
}

#[test]
fn test_add_item_duplicate() {
    let mut flags = Flags { span: Span { start: Position(0), end: Position(3) }, items: vec![] };
    let item = FlagsItem { span: Span { start: Position(0), end: Position(1) }, kind: FlagsItemKind::Flag(Flag::A) };
    flags.add_item(item.clone());
    flags.add_item(item);
}

#[test]
fn test_add_multiple_unique_items() {
    let mut flags = Flags { span: Span { start: Position(0), end: Position(10) }, items: vec![] };
    for i in 0..10 {
        let item = FlagsItem { span: Span { start: Position(i), end: Position(i + 1) }, kind: FlagsItemKind::Flag(Flag::from(i)) };
        flags.add_item(item);
    }
}

#[test]
fn test_add_item_to_full_flags() {
    let mut flags = Flags { span: Span { start: Position(0), end: Position(10) }, items: Vec::new() };
    for i in 0..1000 {
        let item = FlagsItem { span: Span { start: Position(i), end: Position(i + 1) }, kind: FlagsItemKind::Flag(Flag::from(i)) };
        flags.add_item(item);
    }
    let additional_item = FlagsItem { span: Span { start: Position(1000), end: Position(1001) }, kind: FlagsItemKind::Flag(Flag::from(1000)) };
    flags.add_item(additional_item);
}

