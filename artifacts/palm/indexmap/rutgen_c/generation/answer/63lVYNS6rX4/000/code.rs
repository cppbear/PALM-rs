// Answer 0

#[test]
fn test_fmt_debug_index_set() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::RandomState; // Just a placeholder for our test
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::RandomState::new()
        }
    }

    let index_set: super::IndexSet<i32, TestHasher> = super::IndexSet::with_capacity_and_hasher(10, TestHasher);
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        index_set.fmt(&mut formatter).unwrap();
    }

    // Check that the output contains expected debug format for empty IndexSet
    assert!(output.contains("IndexSet"));
    assert!(output.contains("map"));
}

