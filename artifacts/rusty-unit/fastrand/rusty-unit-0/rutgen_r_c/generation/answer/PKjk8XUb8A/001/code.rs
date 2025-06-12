// Answer 0

#[test]
fn test_uppercase_non_empty_choice() {
    let mut rng = Rng::with_seed(1);
    let result = rng.uppercase();
    assert!(result.is_ascii_uppercase());
}

#[test]
#[should_panic(expected = "empty range")]
fn test_uppercase_empty_choice() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let mut rng = Rng::with_seed(2);
    let choice = |_: &[u8]| EmptyIterator {};
    rng.choice = choice; // Assume we can set the choice function for testing
    rng.uppercase();
}

#[test]
fn test_uppercase_random_character() {
    let mut rng = Rng::with_seed(3);
    let results: Vec<char> = (0..1000).map(|_| rng.uppercase()).collect();
    for result in results {
        assert!(result.is_ascii_uppercase());
    }
}

