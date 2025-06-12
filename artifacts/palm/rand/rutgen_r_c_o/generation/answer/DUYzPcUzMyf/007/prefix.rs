// Answer 0

#[test]
fn test_weighted_index_creation_valid_weights() {
    struct WeightType;
    impl Weight for WeightType {
        const ZERO: Self = WeightType;
        fn checked_add_assign(&mut self, _v: &Self) -> Result<(), ()> {
            Ok(())
        }
    }

    struct SampleUniformImpl;
    impl SampleUniform for SampleUniformImpl {
        type Sampler = ();
    }

    let weights: Vec<&WeightType> = vec![&WeightType, &WeightType];
    let result = WeightedIndex::<WeightType>::new(weights);
}

#[test]
fn test_weighted_index_creation_non_negative_weights() {
    struct WeightType;
    impl Weight for WeightType {
        const ZERO: Self = WeightType;
        fn checked_add_assign(&mut self, _v: &Self) -> Result<(), ()> {
            Ok(())
        }
    }

    struct SampleUniformImpl;
    impl SampleUniform for SampleUniformImpl {
        type Sampler = ();
    }

    let weights: Vec<&WeightType> = vec![&WeightType, &WeightType];
    let result = WeightedIndex::<WeightType>::new(weights);
}

#[test]
fn test_weighted_index_creation_with_zero_weights() {
    struct WeightType;
    impl Weight for WeightType {
        const ZERO: Self = WeightType;
        fn checked_add_assign(&mut self, _v: &Self) -> Result<(), ()> {
            Err(())
        }
    }

    struct SampleUniformImpl;
    impl SampleUniform for SampleUniformImpl {
        type Sampler = ();
    }

    let weights: Vec<&WeightType> = vec![&WeightType];
    let result = WeightedIndex::<WeightType>::new(weights);
}

#[test]
fn test_weighted_index_creation_sum_not_zero() {
    struct WeightType;
    impl Weight for WeightType {
        const ZERO: Self = WeightType;
        fn checked_add_assign(&mut self, _v: &Self) -> Result<(), ()> {
            Ok(())
        }
    }

    struct SampleUniformImpl;
    impl SampleUniform for SampleUniformImpl {
        type Sampler = ();
    }

    let weights: Vec<&WeightType> = vec![&WeightType, &WeightType];
    let result = WeightedIndex::<WeightType>::new(weights);
}

