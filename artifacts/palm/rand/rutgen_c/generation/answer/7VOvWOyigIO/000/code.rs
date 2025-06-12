// Answer 0

#[test]
fn test_sample_single_with_non_empty_range() {
    struct MockRng;
    impl RngCore for MockRng {
        // Implement required methods for MockRng...
    }

    let mut rng = MockRng;
    let range = Range { start: 1u32, end: 10u32 };
    let result = range.sample_single(&mut rng);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value >= 1 && value < 10);
}

#[test]
fn test_sample_single_with_empty_range() {
    struct MockRng;
    impl RngCore for MockRng {
        // Implement required methods for MockRng...
    }

    let mut rng = MockRng;
    let range = Range { start: 10u32, end: 10u32 };
    let result = range.sample_single(&mut rng);
    assert_eq!(result, Err(Error::EmptyRange));
}

#[test]
fn test_is_empty_on_non_empty_range() {
    let range = Range { start: 2u32, end: 5u32 };
    assert!(!range.is_empty());
}

#[test]
fn test_is_empty_on_empty_range() {
    let range = Range { start: 10u32, end: 10u32 };
    assert!(range.is_empty());
}

