// Answer 0

#[test]
fn test_is_none_should_return_true_when_index_is_none() {
    let pos = Pos::none();
    assert!(pos.is_none());
}

#[test]
fn test_is_none_should_return_false_when_index_is_some() {
    let pos = Pos::new(1, HashValue(42));
    assert!(!pos.is_none());
}

#[test]
fn test_is_none_should_return_false_when_index_is_zero() {
    let pos = Pos::new(0, HashValue(42));
    assert!(!pos.is_none());
}

