// Answer 0

#[test]
fn test_into_mut() {
    struct Entry<'a, V> {
        index: usize,
        entries: &'a mut Vec<EntryValue<V>>,
    }
    
    struct EntryValue<V> {
        value: V,
    }

    impl<'a, V> Entry<'a, V> {
        pub fn index(&self) -> usize {
            self.index
        }

        pub fn into_mut(self) -> &'a mut V {
            let index = self.index();
            &mut self.entries[index].value
        }
    }

    let mut values = vec![EntryValue { value: 1 }, EntryValue { value: 2 }];
    let entry = Entry { index: 0, entries: &mut values };

    let value_mut: &mut i32 = entry.into_mut();
    *value_mut += 10;

    assert_eq!(values[0].value, 11);
}

#[test]
fn test_into_mut_boundary_case() {
    struct Entry<'a, V> {
        index: usize,
        entries: &'a mut Vec<EntryValue<V>>,
    }
    
    struct EntryValue<V> {
        value: V,
    }

    impl<'a, V> Entry<'a, V> {
        pub fn index(&self) -> usize {
            self.index
        }

        pub fn into_mut(self) -> &'a mut V {
            let index = self.index();
            &mut self.entries[index].value
        }
    }

    let mut values = vec![EntryValue { value: 0 }];
    let entry = Entry { index: 0, entries: &mut values };

    let value_mut: &mut i32 = entry.into_mut();
    *value_mut = -5;

    assert_eq!(values[0].value, -5);
}

