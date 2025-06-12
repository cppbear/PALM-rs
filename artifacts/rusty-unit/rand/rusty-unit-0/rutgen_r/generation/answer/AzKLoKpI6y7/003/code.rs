// Answer 0

#[test]
fn test_partial_shuffle_length_max() {
    use rand::Rng;
    use std::slice;

    struct MockRng;

    impl Rng for MockRng {
        fn gen_range(&mut self, range: std::ops::Range<u32>) -> u32 {
            range.start // Mock implementation
        }
    }

    let mut data: Vec<u32> = (0..u32::MAX as usize).map(|x| x as u32).collect();
    let amount = 1; // Arbitrary non-zero amount to shuffle
    let mut rng = MockRng;

    let (shuffled_part, remaining_part) = data.partial_shuffle(&mut rng, amount);

    assert_eq!(shuffled_part.len(), amount);
    assert_eq!(remaining_part.len(), (u32::MAX as usize) - amount);
}

#[test]
#[should_panic]
fn test_partial_shuffle_panic_on_invalid_swap() {
    use rand::Rng;

    struct MockRng;

    impl Rng for MockRng {
        fn gen_range(&mut self, range: std::ops::Range<u32>) -> u32 {
            // Will lead to panic due to swap conditions
            std::u32::MAX // Mock implementation that exceeds bounds
        }
    }

    let mut data: Vec<u32> = (1..=10).collect();
    let amount = 10; // This forces the entire array to be shuffled
    let mut rng = MockRng;

    let _ = data.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_empty_array() {
    use rand::Rng;

    struct MockRng;

    impl Rng for MockRng {
        fn gen_range(&mut self, _: std::ops::Range<u32>) -> u32 {
            0 // No effect since the array is empty
        }
    }

    let mut data: Vec<u32> = Vec::new();
    let amount = 0; // No elements to shuffle
    let mut rng = MockRng;

    let (shuffled_part, remaining_part) = data.partial_shuffle(&mut rng, amount);

    assert_eq!(shuffled_part.len(), 0);
    assert_eq!(remaining_part.len(), 0);
}

