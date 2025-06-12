// Answer 0

#[test]
fn test_index_empty_map() {
    struct MockMap {
        indices: Vec<()>,
    }
    
    impl MockMap {
        fn new() -> Self {
            MockMap { indices: Vec::new() }
        }
    }

    let map = MockMap::new();
    let index = map.index();
    assert_eq!(index, 0);
}

#[test]
fn test_index_non_empty_map() {
    struct MockMap {
        indices: Vec<()>,
    }
    
    impl MockMap {
        fn new() -> Self {
            MockMap { indices: vec![(), (), ()] }
        }
    }

    let map = MockMap::new();
    let index = map.index();
    assert_eq!(index, 3);
}

