// Answer 0

#[test]
fn test_approximate_size_empty_pattern() {
    let search = BoyerMooreSearch::new(vec![]);
    let result = search.approximate_size();
    assert_eq!(result, 256 * std::mem::size_of::<usize>());
}

#[test]
fn test_approximate_size_single_byte_pattern() {
    let search = BoyerMooreSearch::new(vec![b'a']);
    let result = search.approximate_size();
    assert_eq!(result, (1 * std::mem::size_of::<u8>()) + (256 * std::mem::size_of::<usize>()));
}

#[test]
fn test_approximate_size_large_pattern() {
    let pattern = vec![b'a'; 100]; // A pattern with 100 bytes
    let search = BoyerMooreSearch::new(pattern);
    let result = search.approximate_size();
    assert_eq!(result, (100 * std::mem::size_of::<u8>()) + (256 * std::mem::size_of::<usize>()));
} 

#[test]
fn test_approximate_size_boundary_condition() {
    let pattern = vec![b'a'; usize::MAX];
    let search = BoyerMooreSearch::new(pattern);
    let result = search.approximate_size();
    assert_eq!(result, (usize::MAX * std::mem::size_of::<u8>()) + (256 * std::mem::size_of::<usize>()));
}

