// Answer 0

#[test]
fn test_index_into_mut_valid() {
    struct Container<'v> {
        values: &'v mut Vec<serde_json::Value>,
    }

    impl<'v> Container<'v> {
        fn index_into_mut(&self, v: &'v mut serde_json::Value) -> Option<&'v mut serde_json::Value> {
            self.values.iter_mut().find(|x| x == v)
        }
    }

    let mut value_vec = vec![serde_json::json!(1), serde_json::json!(2), serde_json::json!(3)];
    let mut container = Container { values: &mut value_vec };
    let mut value_to_modify = serde_json::json!(2);

    let result = container.index_into_mut(&mut value_to_modify);
    assert!(result.is_some());
    assert_eq!(*result.unwrap(), serde_json::json!(2));
}

#[test]
fn test_index_into_mut_not_found() {
    struct Container<'v> {
        values: &'v mut Vec<serde_json::Value>,
    }

    impl<'v> Container<'v> {
        fn index_into_mut(&self, v: &'v mut serde_json::Value) -> Option<&'v mut serde_json::Value> {
            self.values.iter_mut().find(|x| x == v)
        }
    }

    let mut value_vec = vec![serde_json::json!(1), serde_json::json!(2), serde_json::json!(3)];
    let mut container = Container { values: &mut value_vec };
    let mut value_not_in_vec = serde_json::json!(4);

    let result = container.index_into_mut(&mut value_not_in_vec);
    assert!(result.is_none());
}

