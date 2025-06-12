// Answer 0

#[derive(Default)]
struct SimpleSet<T> {
    map: std::collections::HashMap<T, ()>,
}

impl<T: std::hash::Hash + Eq> SimpleSet<T> {
    unsafe fn insert_unique_unchecked(&mut self, value: T) -> &T {
        self.map.insert(value, ()).unwrap();
        self.map.keys().last().unwrap()
    }
}

#[test]
fn test_insert_unique_unchecked() {
    let mut set: SimpleSet<i32> = SimpleSet::default();
    unsafe {
        let value = set.insert_unique_unchecked(1);
        assert_eq!(*value, 1);
    }
}

#[test]
fn test_insert_unique_unchecked_multiple() {
    let mut set: SimpleSet<i32> = SimpleSet::default();
    unsafe {
        let value1 = set.insert_unique_unchecked(1);
        let value2 = set.insert_unique_unchecked(2);
        assert_eq!(*value1, 1);
        assert_eq!(*value2, 2);
    }
}

#[should_panic]
#[test]
fn test_insert_unique_unchecked_duplicate() {
    let mut set: SimpleSet<i32> = SimpleSet::default();
    unsafe {
        let _ = set.insert_unique_unchecked(1);
        // Inserting a duplicate may cause panic or undefined behavior
        let _duplicate = set.insert_unique_unchecked(1);
    }
}

