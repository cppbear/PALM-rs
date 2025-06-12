// Answer 0

#[test]
fn test_fmt_empty_map() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn iter(&self) -> std::slice::Iter<(i32, i32)> {
            self.entries.iter()
        }
    }

    let map = TestMap { entries: vec![] };
    let output = format!("{:?}", map); 
    assert_eq!(output, "{}"); 
}

#[test]
fn test_fmt_single_entry_map() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn iter(&self) -> std::slice::Iter<(i32, i32)> {
            self.entries.iter()
        }
    }

    let map = TestMap { entries: vec![(1, 2)] };
    let output = format!("{:?}", map);
    assert_eq!(output, "{1: 2}");
}

#[test]
fn test_fmt_multiple_entries_map() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn iter(&self) -> std::slice::Iter<(i32, i32)> {
            self.entries.iter()
        }
    }

    let map = TestMap { entries: vec![(1, 2), (3, 4), (5, 6)] };
    let output = format!("{:?}", map);
    assert_eq!(output, "{1: 2, 3: 4, 5: 6}");
}

#[test]
fn test_fmt_with_duplicates() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn iter(&self) -> std::slice::Iter<(i32, i32)> {
            self.entries.iter()
        }
    }

    let map = TestMap { entries: vec![(1, 2), (1, 3)] };
    let output = format!("{:?}", map);
    assert_eq!(output, "{1: 2, 1: 3}"); // depending on implementation, order might vary
}

