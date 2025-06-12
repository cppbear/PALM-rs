// Answer 0

#[test]
fn test_lowercase_valid() {
    let mut rng = Rng::with_seed(123);
    let result = rng.lowercase();
    assert!(result.is_ascii_lowercase());
}

#[test]
fn test_lowercase_choice_empty() {
    struct EmptyIter;
    impl IntoIterator for EmptyIter {
        type Item = ();
        type IntoIter = std::iter::empty::Empty<()>;

        fn into_iter(self) -> Self::IntoIter {
            std::iter::empty()
        }
    }

    let mut rng = Rng::with_seed(123);
    let original_choice = std::mem::take(&mut rng.choice);
    
    rng.choice = |_: &[_]| None; // Mock to return None
    let result = std::panic::catch_unwind(|| rng.lowercase());
    
    assert!(result.is_err());
    rng.choice = original_choice; 
}

