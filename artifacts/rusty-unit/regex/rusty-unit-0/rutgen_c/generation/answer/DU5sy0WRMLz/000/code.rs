// Answer 0

#[test]
fn test_add_item_success() {
    struct DummyFlag;
    
    let span = Span {
        start: Position(0),
        end: Position(1),
    };

    let mut flags = Flags {
        span,
        items: vec![],
    };

    let item = FlagsItem {
        span,
        kind: FlagsItemKind::Flag(DummyFlag),
    };

    assert_eq!(flags.add_item(item.clone()), None);
    assert_eq!(flags.items.len(), 1);
    assert_eq!(flags.items[0].kind, item.kind);
}

#[test]
fn test_add_item_duplicate() {
    struct DummyFlag;

    let span = Span {
        start: Position(0),
        end: Position(1),
    };

    let mut flags = Flags {
        span,
        items: vec![],
    };

    let item = FlagsItem {
        span,
        kind: FlagsItemKind::Flag(DummyFlag),
    };

    flags.add_item(item.clone());
    let result = flags.add_item(item.clone());

    assert_eq!(result, Some(0));
    assert_eq!(flags.items.len(), 1);
}

#[test]
fn test_add_item_negation() {
    struct DummyFlag;

    let span1 = Span {
        start: Position(0),
        end: Position(1),
    };

    let span2 = Span {
        start: Position(1),
        end: Position(2),
    };

    let mut flags = Flags {
        span: span1,
        items: vec![],
    };

    let item1 = FlagsItem {
        span: span1,
        kind: FlagsItemKind::Flag(DummyFlag),
    };

    let item2 = FlagsItem {
        span: span2,
        kind: FlagsItemKind::Negation,
    };

    assert_eq!(flags.add_item(item1.clone()), None);
    assert_eq!(flags.add_item(item2.clone()), None);
    assert_eq!(flags.items.len(), 2);
    assert_eq!(flags.items[1].kind, item2.kind);
}

