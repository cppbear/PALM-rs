// Answer 0

#[test]
fn test_remove_entry_existing_key() {
    struct TestEquivalent;

    impl Hash for TestEquivalent {
        fn hash<H: Hasher>(&self, _: &mut H) {}
    }

    impl Equivalent<TestEquivalent> for TestEquivalent {
        fn equivalent(&self, _: &TestEquivalent) -> bool { true }
    }

    let mut map: IndexMap<TestEquivalent, i32, ()> = IndexMap { core: IndexMapCore { indices: Indices::new(), entries: Entries::new() }, hash_builder: () };
    map.insert(TestEquivalent, 42);
    
    let result = map.remove_entry(&TestEquivalent);
    assert_eq!(result, Some((TestEquivalent, 42)));
}

#[test]
fn test_remove_entry_non_existing_key() {
    struct TestEquivalent;

    impl Hash for TestEquivalent {
        fn hash<H: Hasher>(&self, _: &mut H) {}
    }

    impl Equivalent<TestEquivalent> for TestEquivalent {
        fn equivalent(&self, _: &TestEquivalent) -> bool { false }
    }

    let mut map: IndexMap<TestEquivalent, i32, ()> = IndexMap { core: IndexMapCore { indices: Indices::new(), entries: Entries::new() }, hash_builder: () };
    
    let result = map.remove_entry(&TestEquivalent);
    assert_eq!(result, None);
}

#[test]
fn test_remove_entry_multiple_entries() {
    struct TestKey;

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, _: &mut H) {}
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, _: &TestKey) -> bool { true }
    }

    let mut map: IndexMap<TestKey, i32, ()> = IndexMap { core: IndexMapCore { indices: Indices::new(), entries: Entries::new() }, hash_builder: () };
    map.insert(TestKey, 1);
    map.insert(TestKey, 2); // Simulating multiple entries with the same key

    let result = map.remove_entry(&TestKey);
    assert_eq!(result, Some((TestKey, 2))); // Assuming the last entry is returned
}

#[test]
#[should_panic]
fn test_remove_entry_panic_on_key() {
    struct TestKey;

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, _: &mut H) {}
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, _: &TestKey) -> bool { true }
    }

    let mut map: IndexMap<TestKey, i32, ()> = IndexMap { core: IndexMapCore { indices: Indices::new(), entries: Entries::new() }, hash_builder: () };
    
    // Directly using panic if trying to remove non-existing key
    map.remove_entry(&TestKey);
}

