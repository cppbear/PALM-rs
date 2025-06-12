// Answer 0


use hashbrown::HashMap;
use std::hash::BuildHasherDefault;
use std::collections::hash_map::DefaultHasher;

struct EntryRef<'a, K, V> {
    // Placeholder for illustration only. Actual implementation may vary.
    occupied: Option<&'a mut V>,
}

impl<'a, K, V> EntryRef<'a, K, V> {
    pub fn or_default(self) -> &'a mut V
    where
        K: std::hash::Hash + From<&'static str>,
        V: Default,
    {
        match self.occupied {
            Some(entry) => entry,
            None => panic!("Entry is vacant, cannot call or_default"),
        }
    }
}

#[test]
fn test_or_default_occupied() {
    let mut map: HashMap<String, Option<u32>> = HashMap::new();
    map.insert("horseland".to_string(), Some(3));

    let entry_ref = EntryRef { occupied: map.get_mut("horseland") }; // simulating EntryRef::Occupied
    let value = entry_ref.or_default();
    assert_eq!(*value, Some(3));
}

#[test]
#[should_panic(expected = "Entry is vacant, cannot call or_default")]
fn test_or_default_vacant() {
    let map: HashMap<String, Option<u32>> = HashMap::new();
    let entry_ref = EntryRef { occupied: map.get_mut("nonexistent") }; // simulating EntryRef::Vacant
    let _value = entry_ref.or_default(); // This should panic
}


