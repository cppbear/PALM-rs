// Answer 0

#[derive(Debug)]
struct TestKey;
#[derive(Debug)]
struct TestValue;

struct OccupiedEntry<'a, K, V> {
    key: K,
    value: V,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, K, V> OccupiedEntry<'a, K, V> {
    fn insert(&mut self, value: V) -> &mut V {
        self.value = value;
        &mut self.value
    }
}

enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant,
}

impl<'a, K, V> Entry<'a, K, V> {
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {
        match self {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            Entry::Vacant => {
                // For the sake of the test, we assume this is initialized regardless of panic.
                OccupiedEntry {
                    key: TestKey,
                    value,
                    phantom: std::marker::PhantomData,
                }
            }
        }
    }
}

#[test]
fn test_insert_entry_occupied() {
    let initial_value = TestValue;
    let initial_entry = OccupiedEntry {
        key: TestKey,
        value: initial_value,
        phantom: std::marker::PhantomData,
    };
    
    let entry = Entry::Occupied(initial_entry);
    let new_value = TestValue;
    
    let result_entry = entry.insert_entry(new_value);
    
    assert_eq!(std::mem::discriminant(&result_entry), std::mem::discriminant(&initial_entry));
}

#[test]
fn test_insert_entry_vacant() {
    let value = TestValue;
    let entry = Entry::Vacant;
    
    let result_entry = entry.insert_entry(value);
    
    assert_eq!(std::mem::discriminant(&result_entry), std::mem::discriminant(&OccupiedEntry {
        key: TestKey,
        value,
        phantom: std::marker::PhantomData,
    }));
}

