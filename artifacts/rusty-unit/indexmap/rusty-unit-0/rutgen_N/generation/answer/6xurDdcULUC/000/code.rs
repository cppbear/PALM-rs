// Answer 0

#[test]
fn test_into_muts() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct Container<'a, K, V> {
        entries: Vec<Entry<K, V>>,
        index: usize,
        _marker: std::marker::PhantomData<&'a (K, V)>,
    }

    impl<'a, K, V> Container<'a, K, V> {
        pub fn new(entries: Vec<Entry<K, V>>, index: usize) -> Self {
            Self {
                entries,
                index,
                _marker: std::marker::PhantomData,
            }
        }

        pub fn index(&self) -> usize {
            self.index
        }

        pub fn into_muts(self) -> (&'a mut K, &'a mut V) {
            let index = self.index();
            let entry = &mut self.entries[index];
            (&mut entry.key, &mut entry.value)
        }
    }

    let mut entry1 = Entry { key: 1, value: "first" };
    let mut container = Container::new(vec![entry1], 0);
    let (key, value) = container.into_muts();

    *key = 2;
    *value = "updated";

    assert_eq!(*key, 2);
    assert_eq!(*value, "updated");
}

