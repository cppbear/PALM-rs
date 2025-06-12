// Answer 0

#[derive(Debug, PartialEq)]
struct Entry<'a, K, V> {
    key: K,
    value: V,
    occupied: bool,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, K, V> Entry<'a, K, V> {
    fn into_mut(self) -> &'a mut V {
        &mut self.value
    }

    fn insert(self, value: V) -> &'a mut V {
        // This function would normally insert the value into some storage.
        // Here we'll just return a mutable reference to a value for testing.
        &mut self.value
    }
}

#[test]
fn test_or_insert_with_occupaied_entry() {
    let mut value = 42;
    let entry = Entry {
        key: "key1",
        value,
        occupied: true,
        _marker: std::marker::PhantomData,
    };

    let result = entry.or_insert_with(|| 100);
    assert_eq!(*result, 42);
}

#[test]
fn test_or_insert_with_vacant_entry() {
    let entry = Entry {
        key: "key2",
        value: 0,
        occupied: false,
        _marker: std::marker::PhantomData,
    };

    let result = entry.or_insert_with(|| 100);
    assert_eq!(*result, 100);
}

