// Answer 0

#[derive(Debug)]
struct Entry<V> {
    value: Option<V>,
}

impl<V> Entry<V> {
    pub fn occupied(value: V) -> Self {
        Entry { value: Some(value) }
    }

    pub fn get_mut(&mut self) -> &mut V {
        self.value.as_mut().unwrap()
    }
}

impl<V> Entry<V> {
    pub fn and_modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        if let Some(ref mut value) = self.value {
            f(value);
        }
        self
    }
}

#[test]
fn test_and_modify_occupied_entry() {
    let entry = Entry::occupied(10);
    let modified_entry = entry.and_modify(|v| *v += 5);
    assert_eq!(modified_entry.get_mut(), &15);
}

#[test]
fn test_and_modify_no_entry() {
    let entry = Entry { value: None };
    let modified_entry = entry.and_modify(|_v| panic!("This should not be called"));
    assert!(modified_entry.value.is_none());
}

