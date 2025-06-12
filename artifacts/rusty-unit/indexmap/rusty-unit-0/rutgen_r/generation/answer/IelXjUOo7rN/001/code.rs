// Answer 0

#[derive(Debug)]
struct VacantEntry<V> {
    value: Option<V>,
}

#[derive(Debug)]
struct OccupiedEntry<V> {
    value: V,
}

enum Entry<V> {
    Vacant(VacantEntry<V>),
    Occupied(OccupiedEntry<V>),
}

impl<V: Default> Entry<V> {
    pub fn or_default(self) -> &'static mut V {
        match self {
            Entry::Occupied(ref mut entry) => {
                let value: &'static mut V = &mut entry.value;
                value
            }
            Entry::Vacant(entry) => {
                entry.value.get_or_insert_with(Default::default)
            }
        }
    }
}

#[test]
fn test_or_default_vacant_entry() {
    let mut value: Option<i32> = None;
    let entry = Entry::Vacant(VacantEntry { value });

    let result = entry.or_default();
    assert_eq!(*result, 0); // i32 default is 0
}

#[test]
fn test_or_default_occupied_entry() {
    let mut entry = OccupiedEntry { value: 10 };
    let occupied_entry = Entry::Occupied(entry);

    let result = occupied_entry.or_default();
    assert_eq!(*result, 10); // verify the result is the existing value
}

