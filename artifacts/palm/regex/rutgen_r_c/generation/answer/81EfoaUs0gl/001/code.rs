// Answer 0

#[test]
fn test_is_none_max_value() {
    let ch = Char(u32::MAX);
    assert!(ch.is_none());
}

#[test]
fn test_is_none_non_max_value() {
    let ch = Char(0);
    assert!(!ch.is_none());

    let ch = Char(1);
    assert!(!ch.is_none());

    let ch = Char(u32::MAX - 1);
    assert!(!ch.is_none());
}

