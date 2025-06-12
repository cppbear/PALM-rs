// Answer 0

#[test]
fn test_new_reseeding_core_valid_threshold() {
    struct MockRrng;
    
    impl MockRrng {
        fn try_from_rng(reseeder: &mut Rsdr) -> Result<MockInner, Rsdr::Error> {
            // Simulate a successful result
            Ok(MockInner)
        }
    }

    struct MockInner;

    let mut reseeder = Rsdr::new(); // Assume this initializes the reseeder
    let threshold: u64 = 10; // A valid threshold greater than 0
    let result = new(threshold, reseeder);
    
    assert!(result.is_ok());
    let reseeding_core = result.unwrap();
    
    assert_eq!(reseeding_core.threshold, 10);
    assert_eq!(reseeding_core.bytes_until_reseed, 10);
}

#[test]
fn test_new_reseeding_core_max_threshold() {
    struct MockRrng;

    impl MockRrng {
        fn try_from_rng(reseeder: &mut Rsdr) -> Result<MockInner, Rsdr::Error> {
            // Simulate a successful result
            Ok(MockInner)
        }
    }

    struct MockInner;

    let mut reseeder = Rsdr::new(); // Assume this initializes the reseeder
    let threshold: u64 = u64::MAX; // Maximum threshold, should clamp to i64::MAX
    let result = new(threshold, reseeder);
    
    assert!(result.is_ok());
    let reseeding_core = result.unwrap();
    
    assert_eq!(reseeding_core.threshold, i64::MAX);
    assert_eq!(reseeding_core.bytes_until_reseed, i64::MAX);
}

#[test]
fn test_new_reseeding_core_zero_threshold() {
    struct MockRrng;

    impl MockRrng {
        fn try_from_rng(reseeder: &mut Rsdr) -> Result<MockInner, Rsdr::Error> {
            // Simulate a successful result
            Ok(MockInner)
        }
    }

    struct MockInner;

    let mut reseeder = Rsdr::new(); // Assume this initializes the reseeder
    let threshold: u64 = 0; // Test with 0 threshold; the function should clamp to i64::MAX
    let result = new(threshold, reseeder);
    
    assert!(result.is_ok());
    let reseeding_core = result.unwrap();
    
    assert_eq!(reseeding_core.threshold, i64::MAX);
    assert_eq!(reseeding_core.bytes_until_reseed, i64::MAX);
}

