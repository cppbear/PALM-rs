// Answer 0

#[test]
fn test_approximate_size_empty_pattern() {
    let search = BoyerMooreSearch::new(vec![]);
    let size = search.approximate_size();
    assert_eq!(size, 2048); // 256 * size_of::<usize> where size_of::<usize> is typically 8 bytes (64-bit)
}

#[test]
fn test_approximate_size_single_byte_pattern() {
    let search = BoyerMooreSearch::new(vec![b'a']);
    let size = search.approximate_size();
    assert_eq!(size, 2056); // 1 * size_of::<u8>() + 256 * size_of::<usize>
}

#[test]
fn test_approximate_size_large_pattern() {
    let search = BoyerMooreSearch::new(vec![b'a'; 100]);
    let size = search.approximate_size();
    assert_eq!(size, 2056 + 100 * std::mem::size_of::<u8>()); // 100 * size_of::<u8>() + 256 * size_of::<usize>
}

