// Answer 0

#[test]
fn test_weighted_index_new_empty_weights() {
    let result: Result<WeightedIndex<TestWeight>, Error> = WeightedIndex::new(vec![]);
    assert_eq!(result, Err(Error::InvalidInput));
}

#[test]
fn test_weighted_index_new_negative_weight() {
    #[derive(Clone, PartialOrd, PartialEq, Debug)]
    struct TestWeight(f64);
    
    impl Weight for TestWeight {
        const ZERO: Self = TestWeight(0.0);
        
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            if self.0 + v.0 > f64::MAX {
                Err(())
            } else {
                self.0 += v.0;
                Ok(())
            }
        }
    }

    let result: Result<WeightedIndex<TestWeight>, Error> = WeightedIndex::new(vec![TestWeight(-1.0)]);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_weighted_index_new_zero_total_weight() {
    #[derive(Clone, PartialOrd, PartialEq, Debug)]
    struct TestWeight(f64);
    
    impl Weight for TestWeight {
        const ZERO: Self = TestWeight(0.0);
        
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            if self.0 + v.0 > f64::MAX {
                Err(())
            } else {
                self.0 += v.0;
                Ok(())
            }
        }
    }

    let result: Result<WeightedIndex<TestWeight>, Error> = WeightedIndex::new(vec![TestWeight(0.0), TestWeight(0.0)]);
    assert_eq!(result, Err(Error::InsufficientNonZero));
}

#[test]
fn test_weighted_index_new_overflow() {
    #[derive(Clone, PartialOrd, PartialEq, Debug)]
    struct TestWeight(u64);
    
    impl Weight for TestWeight {
        const ZERO: Self = TestWeight(0);
        
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            if self.0 > u64::MAX - v.0 {
                Err(())
            } else {
                self.0 += v.0;
                Ok(())
            }
        }
    }

    let result: Result<WeightedIndex<TestWeight>, Error> = WeightedIndex::new(vec![TestWeight(u64::MAX), TestWeight(1)]);
    assert_eq!(result, Err(Error::Overflow));
}

