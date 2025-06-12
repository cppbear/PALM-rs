// Answer 0

#[test]
fn test_alphabetic_valid() {
    let mut rng = Rng::with_seed(42);
    let result = rng.alphabetic();
    assert!(result.is_ascii_alphabetic()); // Char must be in a-z or A-Z
}

#[test]
#[should_panic(expected = "empty range: ...")]
fn test_alphabetic_empty_choice() {
    struct EmptyChoice;

    impl IntoIterator for EmptyChoice {
        type Item = ();
        type IntoIter = std::iter::Empty<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            std::iter::empty()
        }
    }

    let mut rng = Rng::with_seed(42);
    // Temporarily replace the choice method to simulate an empty choice scenario
    let original_choice = std::mem::replace(
        &mut rng.choice,
        |_: &mut Rng, _: &dyn IntoIterator| -> Option<u8> { None },
    );
    let _ = rng.choice(&EmptyChoice {});
    
    // Restore the original method after 
    std::mem::replace(&mut rng.choice, original_choice);
} 

#[test]
fn test_alphabetic_repeated_calls() {
    let mut rng = Rng::with_seed(123);
    let results: Vec<char> = (0..10).map(|_| rng.alphabetic()).collect();
    
    // Check if all results are valid alphabetic characters
    for &result in &results {
        assert!(result.is_ascii_alphabetic());
    }
}

#[test]
fn test_alphabetic_characters() {
    let mut rng = Rng::with_seed(456);
    
    let chars: Vec<char> = (0..100).map(|_| rng.alphabetic()).collect();
    
    // Ensure the characters cover the expected range
    let valid_chars: Vec<char> = (b'A'..=b'Z')
        .chain(b'a'..=b'z')
        .map(|c| c as char)
        .collect();
    
    for &c in &chars {
        assert!(valid_chars.contains(&c));
    }
}

