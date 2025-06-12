// Answer 0

#[derive(Debug)]
struct Entry<V> {
    value: Option<V>,
}

impl<V> Entry<V> {
    fn new(value: V) -> Self {
        Entry {
            value: Some(value),
        }
    }

    fn get_mut(&mut self) -> &mut V {
        self.value.as_mut().unwrap()
    }

    pub fn insert(&mut self, value: V) -> V {
        std::mem::replace(self.get_mut(), value)
    }
}

#[test]
fn test_insert_replaces_value() {
    let mut entry = Entry::new(42);
    let old_value = entry.insert(100);
    assert_eq!(old_value, 42);
    assert_eq!(*entry.get_mut(), 100);
}

#[test]
fn test_insert_with_string() {
    let mut entry = Entry::new(String::from("Hello"));
    let old_value = entry.insert(String::from("World"));
    assert_eq!(old_value, "Hello");
    assert_eq!(*entry.get_mut(), "World");
}

#[test]
fn test_insert_with_boundary_conditions() {
    let mut entry = Entry::new(0);
    let old_value = entry.insert(usize::MAX);
    assert_eq!(old_value, 0);
    assert_eq!(*entry.get_mut(), usize::MAX);
}

