// Answer 0

#[test]
fn test_select_guard_all_unique() {
    let pattern = vec![b'a', b'b', b'c', b'd'];
    let (guard, index) = BoyerMooreSearch::select_guard(&pattern);
    assert_eq!(guard, b'a');
    assert_eq!(index, 3);
}

#[test]
fn test_select_guard_repeated_characters() {
    let pattern = vec![b'a', b'b', b'a', b'c'];
    let (guard, index) = BoyerMooreSearch::select_guard(&pattern);
    assert_eq!(guard, b'c');
    assert_eq!(index, 3);
}

#[test]
fn test_select_guard_single_character() {
    let pattern = vec![b'a'];
    let (guard, index) = BoyerMooreSearch::select_guard(&pattern);
    assert_eq!(guard, b'a');
    assert_eq!(index, 0);
}

#[test]
fn test_select_guard_with_frequency_table() {
    let pattern = vec![b'a', b'b', b'c', b'a', b'd'];
    let (guard, index) = BoyerMooreSearch::select_guard(&pattern);
    assert_eq!(guard, b'b'); // Assuming b has the lowest frequency based on the frequency table
    assert_eq!(index, 1); // The last occurrence of 'b'
}

