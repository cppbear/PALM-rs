// Answer 0

#[derive(Debug)]
struct Occupied<'a, V> {
    value: &'a mut V,
}

impl<'a, V> Occupied<'a, V> {
    fn into_mut(self) -> &'a mut V {
        self.value
    }
}

#[derive(Debug)]
struct Vacant<'a, V> {
    value: Option<V>,
}

impl<'a, V> Vacant<'a, V> {
    fn insert(self, value: V) -> &'a mut V {
        // In a real scenario, this would properly manage the storage.
        Box::leak(Box::new(value))
    }
}

enum Entry<'a, V> {
    Occupied(Occupied<'a, V>),
    Vacant(Vacant<'a, V>),
}

#[test]
fn test_or_insert_with_vacant() {
    let mut value = 0;
    let entry = Entry::Vacant(Vacant { value: None });
    
    let result = match entry {
        Entry::Occupied(entry) => entry.into_mut(),
        Entry::Vacant(entry) => entry.insert(42),
    };

    assert_eq!(*result, 42);
}

#[test]
fn test_or_insert_with_occupied() {
    let mut value = 42;
    let entry = Entry::Occupied(Occupied { value: &mut value });

    let result = match entry {
        Entry::Occupied(entry) => entry.into_mut(),
        Entry::Vacant(entry) => entry.insert(99),
    };

    assert_eq!(*result, 42); // Should return mutable reference to the existing value
}

