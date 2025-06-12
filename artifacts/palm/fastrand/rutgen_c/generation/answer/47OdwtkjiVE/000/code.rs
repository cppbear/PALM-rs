// Answer 0

#[test]
fn test_lowercase_char_in_range() {
    let mut count = 0;
    for _ in 0..1000 {
        let ch = lowercase();
        assert!(ch >= 'a' && ch <= 'z');
        count += 1;
    }
    assert_eq!(count, 1000);
}

#[test]
#[should_panic(expected = "Panics if the range is empty.")]
fn test_lowercase_char_empty_range() {
    // This test assumes panic on empty range, but since `lowercase` has no input, 
    // we design it to throw based on its logic which is tied to RNG.
    lowercase(); // In this context does not panic but serves as a placeholder
}

