// Answer 0

#[test]
fn test_weighted_index_new_valid_weights() {
    struct SampleWeight(u32);
    
    impl SampleBorrow<SampleWeight> for SampleWeight {
        fn borrow(&self) -> &SampleWeight {
            self
        }
    }
    
    impl Weight for SampleWeight {
        const ZERO: Self = SampleWeight(0);
        
        fn checked_add_assign(&mut self, other: &SampleWeight) -> Result<(), ()> {
            let sum = self.0.checked_add(other.0).ok_or(())?;
            self.0 = sum;
            Ok(())
        }
        
        // Implementing any other necessary methods for Weight trait...
    }
    
    struct Sampler;

    impl Sampler {
        fn new(zero: SampleWeight, total: SampleWeight) -> Result<Self, ()> {
            if total.0 == 0 {
                Err(())
            } else {
                Ok(Sampler)
            }
        }
    }

    let weights = vec![SampleWeight(1), SampleWeight(2), SampleWeight(3)];
    let result = new(weights);
    
    assert!(result.is_ok());

    if let Ok(weighted_index) = result {
        let expected_cumulative_weights = vec![SampleWeight(1), SampleWeight(2), SampleWeight(3)];
        assert_eq!(weighted_index.cumulative_weights, expected_cumulative_weights);
        assert_eq!(weighted_index.total_weight, SampleWeight(6));
    }
}

#[test]
#[should_panic(expected = "InvalidInput")]
fn test_weighted_index_new_empty_weights() {
    let weights: Vec<SampleWeight> = vec![];
    let _ = new(weights).unwrap();
}

#[test]
#[should_panic(expected = "InvalidWeight")]
fn test_weighted_index_new_negative_weight() {
    struct SampleWeight(i32);
    
    impl SampleBorrow<SampleWeight> for SampleWeight {
        fn borrow(&self) -> &SampleWeight {
            self
        }
    }
    
    impl Weight for SampleWeight {
        const ZERO: Self = SampleWeight(0);
        
        fn checked_add_assign(&mut self, other: &SampleWeight) -> Result<(), ()> {
            let sum = self.0.checked_add(other.0).ok_or(())?;
            self.0 = sum;
            Ok(())
        }
        
        // Implementing any other necessary methods for Weight trait...
    }
    
    let weights = vec![SampleWeight(1), SampleWeight(-1)];
    let _ = new(weights).unwrap();
}

#[test]
#[should_panic(expected = "InsufficientNonZero")]
fn test_weighted_index_new_zero_total_weight() {
    let weights = vec![SampleWeight(0), SampleWeight(0)];
    let _ = new(weights).unwrap();
}

#[test]
#[should_panic(expected = "Overflow")]
fn test_weighted_index_new_weights_overflow() {
    struct SampleWeight(u64);
    
    impl SampleBorrow<SampleWeight> for SampleWeight {
        fn borrow(&self) -> &SampleWeight {
            self
        }
    }
    
    impl Weight for SampleWeight {
        const ZERO: Self = SampleWeight(0);
        
        fn checked_add_assign(&mut self, other: &SampleWeight) -> Result<(), ()> {
            let sum = self.0.checked_add(other.0).ok_or(())?;
            self.0 = sum;
            Ok(())
        }
        
        // Implementing any other necessary methods for Weight trait...
    }
    
    let weights = vec![SampleWeight(u64::MAX), SampleWeight(1)];
    let _ = new(weights).unwrap();
}

