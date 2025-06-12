// Answer 0

#[test]
fn test_index_into_valid_case() {
    struct Indexer {
        data: Vec<usize>,
    }

    impl Indexer {
        fn index_into<'v>(&self, v: &'v serde_json::Value) -> Option<&'v serde_json::Value> {
            self.data[..].index_into(v)
        }
    }

    let indexer = Indexer { data: vec![0, 1, 2] };
    let value = serde_json::json!([0, 1, 2, 3]);
    let result = indexer.index_into(&value);
    assert_eq!(result, Some(&value[0]));
}

#[test]
fn test_index_into_empty_case() {
    struct Indexer {
        data: Vec<usize>,
    }

    impl Indexer {
        fn index_into<'v>(&self, v: &'v serde_json::Value) -> Option<&'v serde_json::Value> {
            self.data[..].index_into(v)
        }
    }

    let indexer = Indexer { data: vec![] };
    let value = serde_json::json!([0]);
    let result = indexer.index_into(&value);
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_index_into_out_of_bounds_case() {
    struct Indexer {
        data: Vec<usize>,
    }

    impl Indexer {
        fn index_into<'v>(&self, v: &'v serde_json::Value) -> Option<&'v serde_json::Value> {
            self.data[..].index_into(v)
        }
    }

    let indexer = Indexer { data: vec![0] };
    let value = serde_json::json!([1]);
    let _result = indexer.index_into(&value);
}

