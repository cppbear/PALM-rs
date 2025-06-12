// Answer 0

#[test]
fn test_alphanumeric_returns_valid_char() {
    let mut rng = Rng::with_seed(42);
    let result = rng.alphanumeric();
    assert!(result.is_ascii_alphanumeric());
}

#[test]
fn test_alphanumeric_choice_functionality() {
    let mut rng = Rng::with_seed(43);
    let chars_count = 1000;
    let mut counts = [0; 62]; // 26 lowercase + 26 uppercase + 10 digits
    for _ in 0..chars_count {
        let result = rng.alphanumeric();
        match result {
            'A'..='Z' => counts[(result as usize) - ('A' as usize)] += 1,
            'a'..='z' => counts[26 + (result as usize) - ('a' as usize)] += 1,
            '0'..='9' => counts[52 + (result as usize) - ('0' as usize)] += 1,
            _ => panic!("Unexpected character generated: {}", result),
        }
    }
    // Ensure all characters have been generated at least once
    for &count in &counts {
        assert!(count > 0);
    }
}

#[test]
#[should_panic(expected = "empty range:")]
fn test_alphanumeric_choice_empty() {
    let mut rng = Rng::with_seed(0);
    let chars: &[u8] = &[];
    let result = rng.choice(chars);
    assert!(result.is_none());
}

