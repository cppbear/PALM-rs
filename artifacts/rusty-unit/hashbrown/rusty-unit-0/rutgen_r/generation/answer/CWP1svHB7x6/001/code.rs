// Answer 0

#[test]
fn test_fmt_empty_set() {
    struct TestSet {
        iter: Vec<(i32, i32)>,
    }

    impl TestSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let entries_iter = self.iter.iter().map(|(k, _)| k);
            f.debug_list().entries(entries_iter).finish()
        }
    }

    let empty_set = TestSet { iter: Vec::new() };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        empty_set.fmt(formatter).unwrap();
    }
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_single_entry() {
    struct TestSet {
        iter: Vec<(i32, i32)>,
    }

    impl TestSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let entries_iter = self.iter.iter().map(|(k, _)| k);
            f.debug_list().entries(entries_iter).finish()
        }
    }

    let single_entry = TestSet { iter: vec![(1, 100)] };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        single_entry.fmt(formatter).unwrap();
    }
    assert_eq!(output, "[1]");
}

#[test]
fn test_fmt_multiple_entries() {
    struct TestSet {
        iter: Vec<(i32, i32)>,
    }

    impl TestSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let entries_iter = self.iter.iter().map(|(k, _)| k);
            f.debug_list().entries(entries_iter).finish()
        }
    }

    let multiple_entries = TestSet { iter: vec![(1, 100), (2, 200), (3, 300)] };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        multiple_entries.fmt(formatter).unwrap();
    }
    assert_eq!(output, "[1, 2, 3]");
}

#[test]
fn test_fmt_with_duplicates() {
    struct TestSet {
        iter: Vec<(i32, i32)>,
    }

    impl TestSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let entries_iter = self.iter.iter().map(|(k, _)| k);
            f.debug_list().entries(entries_iter).finish()
        }
    }

    let with_duplicates = TestSet { iter: vec![(1, 100), (1, 200), (2, 300)] };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        with_duplicates.fmt(formatter).unwrap();
    }
    assert_eq!(output, "[1, 1, 2]");
}

