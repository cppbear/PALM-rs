// Answer 0

#[test]
fn test_select_guard_basic_case() {
    let pattern: &[u8] = &[b'a', b'b', b'c', b'a'];
    let (rarest, rarest_rev_idx) = select_guard(pattern);
    assert_eq!(rarest, b'b');
    assert_eq!(rarest_rev_idx, 2);
}

#[test]
fn test_select_guard_all_unique() {
    let pattern: &[u8] = &[b'a', b'b', b'c', b'd'];
    let (rarest, rarest_rev_idx) = select_guard(pattern);
    assert_eq!(rarest, b'a');
    assert_eq!(rarest_rev_idx, 3);
}

#[test]
fn test_select_guard_identical_characters() {
    let pattern: &[u8] = &[b'a', b'a', b'a', b'a'];
    let (rarest, rarest_rev_idx) = select_guard(pattern);
    assert_eq!(rarest, b'a');
    assert_eq!(rarest_rev_idx, 3);
}

#[test]
fn test_select_guard_frequency_case() {
    let pattern: &[u8] = &[b'a', b'b', b'a', b'b', b'a'];
    let (rarest, rarest_rev_idx) = select_guard(pattern);
    assert_eq!(rarest, b'b');
    assert_eq!(rarest_rev_idx, 3);
}

#[test]
#[should_panic]
fn test_select_guard_empty() {
    let pattern: &[u8] = &[];
    select_guard(pattern);
}

#[test]
fn test_select_guard_single_character() {
    let pattern: &[u8] = &[b'a'];
    let (rarest, rarest_rev_idx) = select_guard(pattern);
    assert_eq!(rarest, b'a');
    assert_eq!(rarest_rev_idx, 0);
}

