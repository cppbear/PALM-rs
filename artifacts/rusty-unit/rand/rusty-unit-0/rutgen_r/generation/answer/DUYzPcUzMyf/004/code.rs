// Answer 0

#[test]
fn test_new_weighted_index_overflow() {
    struct MockWeight;
    impl Weight for MockWeight {
        const ZERO: Self = MockWeight;
        
        fn borrow(&self) -> &Self {
            self
        }

        fn checked_add_assign(&mut self, _: &Self) -> Result<(), ()> {
            Err(())
        }
        
        // Implement other necessary methods as needed
    }

    let weights = vec![MockWeight, MockWeight];
    
    let result = new(weights);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::Overflow);
}

#[test]
fn test_new_weighted_index_invalid_input() {
    struct MockWeight;
    impl Weight for MockWeight {
        const ZERO: Self = MockWeight;

        // Implement necessary methods for validation
    }

    let weights: Vec<MockWeight> = vec![];

    let result = new(weights);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidInput);
}

#[test]
fn test_new_weighted_index_invalid_weight() {
    struct MockWeight;
    impl Weight for MockWeight {
        const ZERO: Self = MockWeight;
        
        fn borrow(&self) -> &Self {
            self
        }
        
        // Implement methods to simulate invalid weight
    }

    let weights = vec![MockWeight, MockWeight]; // Initialize with invalid weights

    let result = new(weights);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidWeight);
}

#[test]
fn test_new_weighted_index_insufficient_non_zero() {
    struct MockWeight;
    impl Weight for MockWeight {
        const ZERO: Self = MockWeight;

        fn borrow(&self) -> &Self {
            self
        }
        
        // Implement methods to simulate insufficient non-zero
    }

    let weights = vec![MockWeight, MockWeight]; // Set up weights to sum to zero

    let result = new(weights);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InsufficientNonZero);
}

