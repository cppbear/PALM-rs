// Answer 0

#[test]
fn test_append_string_empty() {
    use crate::RngCore; // Assuming a RngCore trait exists with necessary methods
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    struct DummyRng {
        inner: StdRng,
    }

    impl Rng for DummyRng {
        // Implement necessary methods of the Rng trait here as needed
    }
    
    let mut rng = DummyRng {
        inner: StdRng::from_seed([0; 32]),
    };
    let mut result_string = String::new();
    let alphabetic = Alphabetic;

    alphabetic.append_string(&mut rng, &mut result_string, 0);
    assert_eq!(result_string.len(), 0);
}

#[test]
fn test_append_string_non_zero_length() {
    use crate::RngCore; // Assuming a RngCore trait exists with necessary methods
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    struct DummyRng {
        inner: StdRng,
    }

    impl Rng for DummyRng {
        // Implement necessary methods of the Rng trait here as needed
    }

    let mut rng = DummyRng {
        inner: StdRng::from_seed([1; 32]),
    };
    let mut result_string = String::new();
    let alphabetic = Alphabetic;

    alphabetic.append_string(&mut rng, &mut result_string, 10);
    assert_eq!(result_string.len(), 10);
    // Further checks on the content of result_string can be added if necessary.
}

#[test]
#[should_panic(expected = "capacity overflow")]
fn test_append_string_exceeding_capacity() {
    use crate::RngCore; // Assuming a RngCore trait exists with necessary methods
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    struct DummyRng {
        inner: StdRng,
    }

    impl Rng for DummyRng {
        // Implement necessary methods of the Rng trait here as needed
    }

    let mut rng = DummyRng {
        inner: StdRng::from_seed([2; 32]),
    };
    let mut result_string = String::new();
    let alphabetic = Alphabetic;

    let len = usize::MAX; // This should cause a panic due to overflow when reserving
    alphabetic.append_string(&mut rng, &mut result_string, len);
}

