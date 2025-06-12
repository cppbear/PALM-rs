// Answer 0

#[test]
fn test_new_invalid_weight_negative() {
    use crate::distr::uniform::{Uniform, UniformSampler};
    use crate::distr::Distribution;
    
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TestWeight(f32);

    impl Weight for TestWeight {
        const ZERO: Self = TestWeight(0.0);
        
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            let new_value = self.0 + v.0;
            if new_value.is_infinite() || new_value.is_nan() {
                Err(())
            } else {
                self.0 = new_value;
                Ok(())
            }
        }
    }

    impl SampleUniform for TestWeight {
        type Sampler = Uniform<Self>;
    }

    let weights = vec![TestWeight(-1.0)];
    let result = WeightedIndex::<TestWeight>::new(weights);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_new_invalid_weight_nan() {
    use crate::distr::uniform::{Uniform, UniformSampler};
    
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TestWeight(f32);

    impl Weight for TestWeight {
        const ZERO: Self = TestWeight(0.0);
        
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            let new_value = self.0 + v.0;
            if new_value.is_infinite() || new_value.is_nan() {
                Err(())
            } else {
                self.0 = new_value;
                Ok(())
            }
        }
    }

    impl SampleUniform for TestWeight {
        type Sampler = Uniform<Self>;
    }

    let weights = vec![TestWeight(f32::NAN)];
    let result = WeightedIndex::<TestWeight>::new(weights);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_new_invalid_weight_zero() {
    use crate::distr::uniform::{Uniform, UniformSampler};

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TestWeight(f32);

    impl Weight for TestWeight {
        const ZERO: Self = TestWeight(0.0);
        
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            let new_value = self.0 + v.0;
            if new_value.is_infinite() || new_value.is_nan() {
                Err(())
            } else {
                self.0 = new_value;
                Ok(())
            }
        }
    }

    impl SampleUniform for TestWeight {
        type Sampler = Uniform<Self>;
    }

    let weights = vec![TestWeight(0.0), TestWeight(0.0)];
    let result = WeightedIndex::<TestWeight>::new(weights);
    assert_eq!(result, Err(Error::InsufficientNonZero));
}

