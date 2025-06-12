// Answer 0

#[test]
fn test_into_muts_valid_case() {
    struct Entry<'a, K, V> {
        key: &'a mut K,
        value: &'a mut V,
    }

    struct Container<'a, K, V> {
        entries: Vec<Entry<'a, K, V>>,
    }

    impl<'a, K, V> Container<'a, K, V> {
        fn index(&self) -> usize {
            // Returning a valid index for testing
            0
        }

        fn into_muts(self) -> (&'a mut K, &'a mut V) {
            let index = self.index();
            let entry = &mut self.entries[index];
            (entry.key, entry.value)
        }
    }

    let mut key = 10;
    let mut value = 20;
    let entries = vec![Entry { key: &mut key, value: &mut value }];
    let container = Container { entries };

    let (mut_k, mut_v) = container.into_muts();
    *mut_k += 5;
    *mut_v += 10;

    assert_eq!(*mut_k, 15);
    assert_eq!(*mut_v, 30);
}

#[should_panic]
#[test]
fn test_into_muts_panic_case() {
    struct Entry<'a, K, V> {
        key: &'a mut K,
        value: &'a mut V,
    }

    struct Container<'a, K, V> {
        entries: Vec<Entry<'a, K, V>>,
    }

    impl<'a, K, V> Container<'a, K, V> {
        fn index(&self) -> usize {
            // Returning an out of bounds index
            1
        }

        fn into_muts(self) -> (&'a mut K, &'a mut V) {
            let index = self.index();
            let entry = &mut self.entries[index]; // This line will panic
            (entry.key, entry.value)
        }
    }

    let mut key = 10;
    let mut value = 20;
    let entries = vec![Entry { key: &mut key, value: &mut value }];
    let container = Container { entries };

    // This call should panic
    container.into_muts();
}

