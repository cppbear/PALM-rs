// Answer 0

#[test]
fn test_swap_remove_existing_value() {
    use std::hash::Hash;
    use indexmap::{IndexSet, Equivalent};

    struct MyValue(i32);

    impl Hash for MyValue {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl PartialEq for MyValue {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Equivalent<MyValue> for MyValue {
        fn equivalent(&self, other: &Self) -> bool {
            self.eq(other)
        }
    }

    let mut set: IndexSet<MyValue> = IndexSet::new();
    set.insert(MyValue(1));
    set.insert(MyValue(2));
    set.insert(MyValue(3));

    assert_eq!(set.swap_remove(&MyValue(2)), true);
    assert_eq!(set.len(), 2);
}

#[test]
fn test_swap_remove_non_existing_value() {
    use std::hash::Hash;
    use indexmap::{IndexSet, Equivalent};

    struct MyValue(i32);

    impl Hash for MyValue {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl PartialEq for MyValue {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Equivalent<MyValue> for MyValue {
        fn equivalent(&self, other: &Self) -> bool {
            self.eq(other)
        }
    }

    let mut set: IndexSet<MyValue> = IndexSet::new();
    set.insert(MyValue(1));
    set.insert(MyValue(2));

    assert_eq!(set.swap_remove(&MyValue(3)), false);
    assert_eq!(set.len(), 2);
}

#[test]
fn test_swap_remove_last_value() {
    use std::hash::Hash;
    use indexmap::{IndexSet, Equivalent};

    struct MyValue(i32);

    impl Hash for MyValue {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl PartialEq for MyValue {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Equivalent<MyValue> for MyValue {
        fn equivalent(&self, other: &Self) -> bool {
            self.eq(other)
        }
    }

    let mut set: IndexSet<MyValue> = IndexSet::new();
    set.insert(MyValue(1));
    set.insert(MyValue(2));

    assert_eq!(set.swap_remove(&MyValue(2)), true);
    assert_eq!(set.len(), 1);
    assert_eq!(set.swap_remove(&MyValue(1)), true);
    assert_eq!(set.len(), 0);
}

#[test]
fn test_swap_remove_on_empty_set() {
    use std::hash::Hash;
    use indexmap::{IndexSet, Equivalent};

    struct MyValue(i32);

    impl Hash for MyValue {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl PartialEq for MyValue {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Equivalent<MyValue> for MyValue {
        fn equivalent(&self, other: &Self) -> bool {
            self.eq(other)
        }
    }

    let mut set: IndexSet<MyValue> = IndexSet::new();

    assert_eq!(set.swap_remove(&MyValue(1)), false);
    assert_eq!(set.len(), 0);
}

