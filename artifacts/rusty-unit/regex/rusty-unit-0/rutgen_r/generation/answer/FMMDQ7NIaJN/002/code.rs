// Answer 0

#[test]
fn test_select_guard_with_non_unique_elements() {
    let pattern = [b'a', b'a', b'a', b'b', b'b', b'b'];
    let (guard, index) = select_guard(&pattern);
    assert_eq!(guard, b'a'); // The most frequent element
    assert_eq!(index, 2); // The reversed index of the last 'a'
}

#[test]
fn test_select_guard_with_single_element() {
    let pattern = [b'x'];
    let (guard, index) = select_guard(&pattern);
    assert_eq!(guard, b'x'); // The only element
    assert_eq!(index, 0); // The reversed index of 'x'
}

#[test]
#[should_panic]
fn test_select_guard_with_empty_pattern() {
    let pattern: &[u8] = &[];
    select_guard(pattern); // This should panic due to indexing an empty slice
}

#[test]
fn test_select_guard_with_all_same_elements() {
    let pattern = [b'c', b'c', b'c', b'c'];
    let (guard, index) = select_guard(&pattern);
    assert_eq!(guard, b'c'); // The only element
    assert_eq!(index, 0); // The reversed index of 'c'
}

#[test]
fn test_select_guard_with_two_different_elements() {
    let pattern = [b'y', b'z'];
    let (guard, index) = select_guard(&pattern);
    assert_eq!(guard, b'y'); // The first one is chosen
    assert_eq!(index, 1); // The reversed index of 'y'
}

