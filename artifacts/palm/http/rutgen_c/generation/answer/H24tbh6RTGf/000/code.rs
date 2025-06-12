// Answer 0

#[test]
fn test_resolve_some() {
    let pos = Pos::new(5, HashValue(42));
    assert_eq!(pos.resolve(), Some((5, HashValue(42))));
}

#[test]
fn test_resolve_none() {
    let pos = Pos::none();
    assert_eq!(pos.resolve(), None);
}

