// Answer 0

#[derive(Debug)]
struct OccupiedEntry<'a, V> {
    value: V,
    phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Debug)]
struct VacantEntry<'a, V> {
    phantom: std::marker::PhantomData<&'a ()>,
}

enum Entry<'a, V> {
    Occupied(OccupiedEntry<'a, V>),
    Vacant(VacantEntry<'a, V>),
}

impl<'a, V> OccupiedEntry<'a, V> {
    fn into_mut(self) -> &'a mut V {
        // Dummy implementation for the purpose of the test
        unsafe { &mut *(std::mem::transmute::<&OccupiedEntry<'a, V>, *mut V>(&self) as *mut V) }
    }
}

impl<'a, V> VacantEntry<'a, V> {
    fn insert(self, default: V) -> &'a mut V {
        // Dummy implementation to simulate insertion
        unsafe { &mut *(std::mem::transmute::<&VacantEntry<'a, V>, *mut V>(&self) as *mut V) }
    }
}

impl<'a, V> Entry<'a, V> {
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }
}

#[test]
fn test_or_insert_occupied() {
    let mut value = 10;
    let entry = Entry::Occupied(OccupiedEntry {
        value,
        phantom: std::marker::PhantomData,
    });

    let result: &mut i32 = entry.or_insert(20);
    assert_eq!(*result, 10);
}

#[test]
fn test_or_insert_vacant() {
    let entry = Entry::Vacant(VacantEntry {
        phantom: std::marker::PhantomData,
    });

    let result: &mut i32 = entry.or_insert(20);
    assert_eq!(*result, 20);
}

