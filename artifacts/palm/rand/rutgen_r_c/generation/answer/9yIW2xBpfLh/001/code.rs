// Answer 0

#[test]
fn test_random_range_float() {
    use rand_core::RngCore;
    use rand::distributions::Uniform;

    struct TestRng;

    impl RngCore for TestRng {
        // Implement necessary methods of RngCore as needed for test
    }

    let low: f32 = 0.0;
    let high: f32 = 1e9;
    let range = Uniform::new(low, high);
    let result: f32 = random_range(range);
    assert!(result >= low && result <= high);
}

#[test]
fn test_random_range_integer() {
    use rand_core::RngCore;
    use rand::distributions::Uniform;

    struct TestRng;

    impl RngCore for TestRng {
        // Implement necessary methods of RngCore as needed for test
    }

    let low: usize = 0;
    let high: usize = 100;
    let range = Uniform::new(low, high);
    let result: usize = random_range(range);
    assert!(result >= low && result < high);
}

#[test]
#[should_panic]
fn test_random_range_empty() {
    use rand_core::RngCore;
    use rand::distributions::Uniform;

    struct TestRng;

    impl RngCore for TestRng {
        // Implement necessary methods of RngCore as needed for test
    }

    let range = Uniform::new(1, 0); // Invalid range
    let _: usize = random_range(range);
}

#[test]
fn test_random_range_string() {
    use rand_core::RngCore;
    use rand::distributions::Uniform;

    struct TestRng;

    impl RngCore for TestRng {
        // Implement necessary methods of RngCore as needed for test
    }

    let words = vec!["Mary", "had", "a", "little", "lamb"];
    let range = Uniform::from(0..words.len());
    let idx: usize = random_range(range);
    assert!(idx < words.len());
}

