// Answer 0

#[test]
fn test_partial_shuffle_u32_max_length() {
    use rand::Rng;
    use rand::thread_rng;

    // Create a slice of length u32::MAX (which is 4294967295)
    let max_length = u32::MAX as usize;
    let mut vec: Vec<u32> = (0..max_length).collect();

    // Create a mutable reference to the slice
    let slice: &mut [u32] = &mut vec;

    // Create a random number generator
    let mut rng = thread_rng();

    // Test with an amount that triggers the constraint self.len() < (u32::MAX as usize) to be false
    let amount = 1;

    // The following should not panic as it satisfies the boundary condition
    let (shuffled_part, remaining_part) = slice.partial_shuffle(&mut rng, amount);

    // Check that the return values are slices with the correct lengths
    assert_eq!(shuffled_part.len(), amount);
    assert_eq!(remaining_part.len(), max_length - amount);
}

#[test]
#[should_panic]
fn test_partial_shuffle_boundary_m_panic() {
    use rand::Rng;
    use rand::thread_rng;

    // Create a slice of length u32::MAX (which is 4294967295)
    let max_length = u32::MAX as usize;
    let mut vec: Vec<u32> = (0..max_length).collect();

    // Create a mutable reference to the slice
    let slice: &mut [u32] = &mut vec;

    // Create a random number generator
    let mut rng = thread_rng();

    // Test with an amount that causes m to be out of bounds
    let amount = max_length; // This will cause m to be 0

    // This should panic due to self.split_at_mut(m)
    let (_, _) = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_no_elements() {
    use rand::Rng;
    use rand::thread_rng;

    // Create an empty slice
    let mut vec: Vec<u32> = Vec::new();
    let slice: &mut [u32] = &mut vec;

    // Create a random number generator
    let mut rng = thread_rng();

    // Test with amount 0
    let amount = 0;

    // This should return two empty slices without panicking
    let (shuffled_part, remaining_part) = slice.partial_shuffle(&mut rng, amount);
    
    assert_eq!(shuffled_part.len(), 0);
    assert_eq!(remaining_part.len(), 0);
}

