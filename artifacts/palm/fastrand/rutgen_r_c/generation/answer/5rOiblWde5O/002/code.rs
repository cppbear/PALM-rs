// Answer 0

#[test]
#[should_panic]
fn test_choice_empty_iterator() {
    struct TestIterator;

    impl IntoIterator for TestIterator {
        type Item = usize;
        type IntoIter = std::iter::empty::Empty<usize>;

        fn into_iter(self) -> Self::IntoIter {
            std::iter::empty()
        }
    }

    let mut rng = Rng::with_seed(42);
    let iterator = TestIterator;
    let result = rng.choice(iterator);
    assert_eq!(result, None);
}

