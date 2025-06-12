// Answer 0

#[derive(Debug)]
struct Entry<'a, V> {
    value: Option<&'a mut V>,
}

impl<'a, V> Entry<'a, V> {
    fn into_mut(self) -> &'a mut V {
        self.value.expect("Expected occupied entry")
    }

    fn vacant() -> Self {
        Entry { value: None }
    }

    fn occupied(value: &'a mut V) -> Self {
        Entry { value: Some(value) }
    }

    fn insert(self, value: V) -> &'a mut V {
        if let Some(val) = self.value {
            val
        } else {
            panic!("Can't insert into occupied entry")
        }
    }
}

#[test]
fn test_or_insert_with_vacant_entry() {
    let mut value = 42;
    let entry = Entry::vacant();

    let result = entry.or_insert_with(|| 100);

    assert_eq!(*result, 100);
}

#[test]
fn test_or_insert_with_occupied_entry() {
    let mut value = 42;
    let entry = Entry::occupied(&mut value);

    let result = entry.or_insert_with(|| 100);

    assert_eq!(*result, 42);
    assert_eq!(*entry.into_mut(), 42);
}

