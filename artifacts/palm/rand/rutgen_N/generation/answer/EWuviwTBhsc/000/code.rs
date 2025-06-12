// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;
    
    struct WeightedIndexIter {
        weighted_index: Vec<usize>,
        index: usize,
    }

    impl fmt::Debug for WeightedIndexIter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("WeightedIndexIter")
                .field("weighted_index", &self.weighted_index)
                .field("index", &self.index)
                .finish()
        }
    }

    let weighted_index = vec![10, 20, 30];
    let index = 1;
    let iter = WeightedIndexIter { weighted_index, index };

    let expected_output = format!("WeightedIndexIter {{ weighted_index: {:?}, index: {} }}", iter.weighted_index, iter.index);
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        iter.fmt(formatter).unwrap();
    }

    assert_eq!(output, expected_output);
}

