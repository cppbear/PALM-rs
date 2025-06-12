// Answer 0

#[derive(Debug, PartialEq, Eq, Hash)]
struct MyValue {
    data: i32,
}

struct MySet {
    map: indexmap::IndexMap<MyValue, ()>,
}

impl MySet {
    fn new() -> Self {
        MySet {
            map: indexmap::IndexMap::new(),
        }
    }

    pub fn shift_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, MyValue)>
    where
        Q: ?Sized + Hash + Equivalent<MyValue>,
    {
        self.map.shift_remove_full(value).map(|(i, x, ())| (i, x))
    }
}

#[test]
fn test_shift_remove_full_existing_value() {
    let mut set = MySet::new();
    set.map.insert(MyValue { data: 1 }, ());
    set.map.insert(MyValue { data: 2 }, ());

    let result = set.shift_remove_full(&MyValue { data: 1 });
    assert_eq!(result, Some((0, MyValue { data: 1 })));
    assert_eq!(set.map.len(), 1); // Ensure size has decreased
}

#[test]
fn test_shift_remove_full_non_existing_value() {
    let mut set = MySet::new();
    set.map.insert(MyValue { data: 1 }, ());

    let result = set.shift_remove_full(&MyValue { data: 2 });
    assert_eq!(result, None);
    assert_eq!(set.map.len(), 1); // Ensure size remains the same
}

#[test]
fn test_shift_remove_full_boundary_case() {
    let mut set = MySet::new();
    set.map.insert(MyValue { data: 1 }, ());
    set.map.insert(MyValue { data: 2 }, ());
    set.map.insert(MyValue { data: 3 }, ());

    let result = set.shift_remove_full(&MyValue { data: 3 });
    assert_eq!(result, Some((2, MyValue { data: 3 })));
    assert_eq!(set.map.len(), 2); // Ensure size has decreased
}

