// Answer 0

#[test]
fn test_sample_valid_index() {
    struct MockRng; // Dummy RNG struct for testing
    impl Rng for MockRng {
        // Implement necessary methods for the RNG trait
    }

    let slice = &["apple", "banana", "cherry"];
    let range = UniformUsize { low: 0, range: slice.len(), thresh: 0, mode64: false }; // Dummy initialization for the sake of testing
    let choose = Choose { slice, range, num_choices: NonZeroUsize::new(1).unwrap() };
    let mut rng = MockRng;

    // Sampling should yield a valid slice reference
    for _ in 0..10 {
        let item = choose.sample(&mut rng);
        assert!(slice.contains(&item));
    }
}

#[test]
#[should_panic(expected = "Uniform::new(0, 0) somehow returned 0")]
fn test_sample_empty_slice() {
    struct MockRng; // Dummy RNG struct for testing
    impl Rng for MockRng {
        // Implement necessary methods for the RNG trait
    }

    let slice: &[&str] = &[];
    let range = UniformUsize { low: 0, range: 0, thresh: 0, mode64: false };
    let choose = Choose { slice, range, num_choices: NonZeroUsize::new(1).unwrap() };
    let mut rng = MockRng;

    // This should panic due to accessing an index in an empty slice
    choose.sample(&mut rng);
}

