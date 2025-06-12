// Answer 0

#[test]
fn test_new_with_non_empty_pattern() {
    let pattern = vec![3, 1, 2, 4, 5]; // Example pattern with multiple bytes
    let packed = FreqyPacked::new(pattern.clone());

    assert_eq!(packed.pat, pattern);
    assert_eq!(packed.char_len, 5);
    assert_eq!(packed.rare1, 1); // Assuming 1 has the lowest frequency
    assert_eq!(packed.rare1i, 1);
    assert_eq!(packed.rare2, 2); // Assuming 2 has the second lowest frequency
    assert_eq!(packed.rare2i, 2);
}

#[test]
fn test_new_with_repeated_bytes() {
    let pattern = vec![7, 1, 7, 2, 1]; // Rarest bytes are repeated
    let packed = FreqyPacked::new(pattern.clone());

    assert_eq!(packed.pat, pattern);
    assert_eq!(packed.char_len, 5);
    assert_eq!(packed.rare1, 1); // Assuming 1 is the rarest
    assert_eq!(packed.rare1i, 4);
    assert_eq!(packed.rare2, 2); // Assuming 2 is the second rarest
    assert_eq!(packed.rare2i, 3);
}

#[test]
fn test_new_with_unique_bytes() {
    let pattern = vec![5, 3, 1, 4, 2]; // All unique bytes
    let packed = FreqyPacked::new(pattern.clone());

    assert_eq!(packed.pat, pattern);
    assert_eq!(packed.char_len, 5);
    assert_eq!(packed.rare1, 1); // Assuming 1 is the rarest
    assert_eq!(packed.rare1i, 2);
    assert_eq!(packed.rare2, 2); // Assuming 2 is the second rarest
    assert_eq!(packed.rare2i, 4);
}

#[test]
fn test_new_with_empty_input() {
    let pattern: Vec<u8> = vec![];
    let packed = FreqyPacked::new(pattern.clone());

    assert_eq!(packed.pat, vec![]);
    assert_eq!(packed.char_len, 0);
    assert_eq!(packed.rare1, 0);
    assert_eq!(packed.rare1i, 0);
    assert_eq!(packed.rare2, 0);
    assert_eq!(packed.rare2i, 0);
}

