// Answer 0

#[test]
fn test_select_guard_with_multiple_characters() {
    let pattern: Vec<u8> = vec![b'a', b'b', b'c', b'd'];
    let (rarest, rarest_rev_idx) = select_guard(&pattern);
    assert_eq!(rarest, b'a');
    assert_eq!(rarest_rev_idx, 3);
}

#[test]
fn test_select_guard_with_single_character() {
    let pattern: Vec<u8> = vec![b'a'];
    let (rarest, rarest_rev_idx) = select_guard(&pattern);
    assert_eq!(rarest, b'a');
    assert_eq!(rarest_rev_idx, 0);
}

#[test]
fn test_select_guard_with_same_characters() {
    let pattern: Vec<u8> = vec![b'b', b'b', b'b'];
    let (rarest, rarest_rev_idx) = select_guard(&pattern);
    assert_eq!(rarest, b'b');
    assert_eq!(rarest_rev_idx, 2);
}

#[test]
fn test_select_guard_with_frequencies() {
    let pattern: Vec<u8> = vec![b'a', b'b', b'a', b'c', b'd', b'b'];
    let (rarest, rarest_rev_idx) = select_guard(&pattern);
    assert_eq!(rarest, b'c');
    assert_eq!(rarest_rev_idx, 3);
}

#[should_panic]
fn test_select_guard_empty_pattern() {
    let pattern: Vec<u8> = vec![];
    select_guard(&pattern);
}

