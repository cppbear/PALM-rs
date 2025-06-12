// Answer 0

#[test]
fn test_get_mut() {
    struct TestEntry<V> {
        entries: Vec<TestValue<V>>,
        index: usize,
    }

    struct TestValue<V> {
        value: V,
    }

    impl<V> TestEntry<V> {
        fn new(entries: Vec<TestValue<V>>, index: usize) -> Self {
            TestEntry { entries, index }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn get_mut(&mut self) -> &mut V {
            let index = self.index();
            &mut self.entries[index].value
        }
    }

    let mut entries = vec![TestValue { value: 1 }, TestValue { value: 2 }];
    let mut entry = TestEntry::new(entries, 0);
    let value_mut = entry.get_mut();
    *value_mut += 1;

    assert_eq!(*value_mut, 2);
}

#[test]
fn test_get_mut_boundary_condition() {
    struct TestEntry<V> {
        entries: Vec<TestValue<V>>,
        index: usize,
    }

    struct TestValue<V> {
        value: V,
    }

    impl<V> TestEntry<V> {
        fn new(entries: Vec<TestValue<V>>, index: usize) -> Self {
            TestEntry { entries, index }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn get_mut(&mut self) -> &mut V {
            let index = self.index();
            &mut self.entries[index].value
        }
    }

    let mut entries = vec![TestValue { value: 1 }];
    let mut entry = TestEntry::new(entries, 0);
    let value_mut = entry.get_mut();
    *value_mut = 10;

    assert_eq!(*value_mut, 10);
}

