// Answer 0

#[test]
fn test_fmt_with_empty_map() {
    struct TestMap {
        items: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn iter(&self) -> std::slice::Iter<(i32, i32)> {
            self.items.iter()
        }
    }

    let test_map = TestMap { items: Vec::new() };
    let result = format!("{:?}", test_map);
    assert_eq!(result, "{}");
}

#[test]
fn test_fmt_with_single_entry() {
    struct TestMap {
        items: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn iter(&self) -> std::slice::Iter<(i32, i32)> {
            self.items.iter()
        }
    }

    let test_map = TestMap { items: vec![(1, 2)] };
    let result = format!("{:?}", test_map);
    assert_eq!(result, "{1: 2}");
}

#[test]
fn test_fmt_with_multiple_entries() {
    struct TestMap {
        items: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn iter(&self) -> std::slice::Iter<(i32, i32)> {
            self.items.iter()
        }
    }

    let test_map = TestMap { items: vec![(1, 2), (3, 4), (5, 6)] };
    let result = format!("{:?}", test_map);
    assert_eq!(result, "{1: 2, 3: 4, 5: 6}");
}

