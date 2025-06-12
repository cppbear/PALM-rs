// Answer 0

#[test]
fn test_pos_new_valid_index() {
    let index = 100;
    let hash = HashValue(42);
    let pos = Pos::new(index, hash);
    assert_eq!(pos.index, index as Size);
    assert_eq!(pos.hash, hash);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_pos_new_invalid_index() {
    let index = MAX_SIZE; // should panic
    let hash = HashValue(42);
    Pos::new(index, hash);
}

#[test]
fn test_pos_none() {
    let pos = Pos::none();
    assert_eq!(pos.index, !0);
    assert_eq!(pos.hash, HashValue(0));
    assert!(!pos.is_some());
    assert!(pos.is_none());
}

#[test]
fn test_pos_is_some() {
    let index = 10;
    let hash = HashValue(5);
    let pos = Pos::new(index, hash);
    assert!(pos.is_some());
}

#[test]
fn test_pos_is_none() {
    let pos = Pos::none();
    assert!(!pos.is_some());
    assert!(pos.is_none());
}

#[test]
fn test_pos_resolve() {
    let index = 15;
    let hash = HashValue(55);
    let pos = Pos::new(index, hash);
    let resolved = pos.resolve();
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap(), (index, hash));
}

