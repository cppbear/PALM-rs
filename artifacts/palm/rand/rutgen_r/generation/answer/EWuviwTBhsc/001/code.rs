// Answer 0

#[test]
fn test_weighted_index_iter_debug_struct() {
    struct WeightedIndexIter {
        weighted_index: Vec<u32>,
        index: usize,
    }

    impl WeightedIndexIter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("WeightedIndexIter")
                .field("weighted_index", &self.weighted_index)
                .field("index", &self.index)
                .finish()
        }
    }

    // Test with typical case
    let iter = WeightedIndexIter {
        weighted_index: vec![1, 2, 3, 4, 5],
        index: 2,
    };
    let mut buf = String::new();
    {
        let result = std::fmt::format(format_args!("{:?}", iter.fmt(&mut std::fmt::Formatter::new())));
        assert!(result.contains("weighted_index"));
        assert!(result.contains("[1, 2, 3, 4, 5]"));
        assert!(result.contains("index"));
        assert!(result.contains("2"));
    }

    // Test with empty weighted_index and index set to zero
    let iter_empty = WeightedIndexIter {
        weighted_index: vec![],
        index: 0,
    };
    let result_empty = std::fmt::format(format_args!("{:?}", iter_empty.fmt(&mut std::fmt::Formatter::new())));
    assert!(result_empty.contains("weighted_index"));
    assert!(result_empty.contains("[]"));
    assert!(result_empty.contains("index"));
    assert!(result_empty.contains("0"));

    // Test with single element in weighted_index
    let iter_single = WeightedIndexIter {
        weighted_index: vec![10],
        index: 0,
    };
    let result_single = std::fmt::format(format_args!("{:?}", iter_single.fmt(&mut std::fmt::Formatter::new())));
    assert!(result_single.contains("weighted_index"));
    assert!(result_single.contains("[10]"));
    assert!(result_single.contains("index"));
    assert!(result_single.contains("0"));
}

