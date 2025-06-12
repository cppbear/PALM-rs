// Answer 0

#[test]
fn test_weighted_index_new_with_negative_weight() {
    #[derive(Debug, Clone, PartialEq, Default)]
    struct TestWeight(i32);

    impl Weight for TestWeight {
        const ZERO: Self = TestWeight(0);
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            let new_value = self.0.checked_add(v.0).ok_or(())?;
            self.0 = new_value;
            Ok(())
        }
    }

    let weights = vec![TestWeight(1), TestWeight(-1)];
    let result = WeightedIndex::<TestWeight>::new(weights);
}

#[test]
fn test_weighted_index_new_with_zero_weight_and_non_zero_first_weight() {
    #[derive(Debug, Clone, PartialEq, Default)]
    struct TestWeight(i32);

    impl Weight for TestWeight {
        const ZERO: Self = TestWeight(0);
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            let new_value = self.0.checked_add(v.0).ok_or(())?;
            self.0 = new_value;
            Ok(())
        }
    }

    let weights = vec![TestWeight(1), TestWeight(0)];
    let result = WeightedIndex::<TestWeight>::new(weights);
}

#[test]
fn test_weighted_index_new_with_negative_weight_only() {
    #[derive(Debug, Clone, PartialEq, Default)]
    struct TestWeight(i32);

    impl Weight for TestWeight {
        const ZERO: Self = TestWeight(0);
        fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
            let new_value = self.0.checked_add(v.0).ok_or(())?;
            self.0 = new_value;
            Ok(())
        }
    }

    let weights = vec![TestWeight(-1)];
    let result = WeightedIndex::<TestWeight>::new(weights);
}

