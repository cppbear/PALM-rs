// Answer 0

#[test]
fn test_split_off_boundary_case() {
    #[derive(Debug)]
    struct TestHashValue(usize);
    impl HashValue for TestHashValue {
        fn get(&self) -> usize {
            self.0
        }
    }

    let mut map_core = IndexMapCore::<usize, String>::with_capacity(5);
    for i in 0..5 {
        map_core.entries.push(Bucket {
            hash: TestHashValue(i),
            key: i,
            value: format!("value{}", i),
        });
    }

    let at = map_core.entries.len();
    let result = map_core.split_off(at);

    assert_eq!(result.entries.len(), 0);
    assert_eq!(map_core.entries.len(), at);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_split_off_out_of_bounds() {
    #[derive(Debug)]
    struct TestHashValue(usize);
    impl HashValue for TestHashValue {
        fn get(&self) -> usize {
            self.0
        }
    }

    let mut map_core = IndexMapCore::<usize, String>::new();
    map_core.entries.push(Bucket {
        hash: TestHashValue(0),
        key: 0,
        value: "value0".to_string(),
    });

    let at = 2; // Out of bounds since we only have 1 entry
    let _result = map_core.split_off(at);
}

