// Answer 0

#[cfg(test)]
fn test_weighted_index_new() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct WeightImpl(i32);
    
    impl Weight for WeightImpl {
        const ZERO: Self = WeightImpl(0);
        
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            let new_value = self.0.checked_add(v.0).ok_or(())?;
            self.0 = new_value;
            Ok(())
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct SampleUniformImpl;

    impl SampleUniform for SampleUniformImpl {
        type Sampler = SampleUniformImpl;
    }

    let result: Result<WeightedIndex<WeightImpl>, Error> = WeightedIndex::new(vec![WeightImpl(0)]);

    assert_eq!(result, Err(Error::InsufficientNonZero));
}

#[test]
fn test_weighted_index_new_empty() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct WeightImpl(i32);
    
    impl Weight for WeightImpl {
        const ZERO: Self = WeightImpl(0);
        
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            let new_value = self.0.checked_add(v.0).ok_or(())?;
            self.0 = new_value;
            Ok(())
        }
    }

    let result: Result<WeightedIndex<WeightImpl>, Error> = WeightedIndex::new(Vec::<WeightImpl>::new());

    assert_eq!(result, Err(Error::InvalidInput));
}

#[test]
fn test_weighted_index_new_negative_weight() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct WeightImpl(i32);
    
    impl Weight for WeightImpl {
        const ZERO: Self = WeightImpl(0);
        
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            let new_value = self.0.checked_add(v.0).ok_or(())?;
            self.0 = new_value;
            Ok(())
        }
    }

    let result: Result<WeightedIndex<WeightImpl>, Error> = WeightedIndex::new(vec![WeightImpl(-1)]);

    assert_eq!(result, Err(Error::InvalidWeight));
}

