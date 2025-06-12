// Answer 0

#[test]
fn test_fill_with_empty_array() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::seed_from_u64(0);
    let mut array: [u32; 0] = [];
    array.fill(&mut rng);
    assert!(array.is_empty());
}

#[test]
fn test_fill_with_single_element_array() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::seed_from_u64(0);
    let mut array = [0u32; 1];
    array.fill(&mut rng);
    assert!(array[0] != 0);  // Assuming rng.random() generates a non-zero value
}

#[test]
fn test_fill_with_multiple_elements_array() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::seed_from_u64(0);
    let mut array = [0u32; 5];
    array.fill(&mut rng);
    assert!(array.iter().any(|&x| x != 0));  // Check if at least one value is non-zero
}

#[test]
fn test_fill_with_non_default_rng() {
    use rand::rngs::ThreadRng;
    use rand::thread_rng;

    let mut rng: ThreadRng = thread_rng();
    let mut array = [0u32; 3];
    array.fill(&mut rng);
    assert!(array.iter().any(|&x| x != 0));  // Check if at least one value is non-zero
}

