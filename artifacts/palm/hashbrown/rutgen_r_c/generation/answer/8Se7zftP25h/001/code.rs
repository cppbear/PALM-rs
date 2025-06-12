// Answer 0

#[test]
fn test_get_found_value() {
    use hashbrown::HashSet;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct DummyHasher(DefaultHasher);

    impl Hash for i32 {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.to_le().hash(state);
        }
    }

    let mut set: HashSet<i32, DummyHasher> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert_eq!(set.get(&2), Some(&2));
}

#[test]
fn test_get_not_found_value() {
    use hashbrown::HashSet;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct DummyHasher(DefaultHasher);

    impl Hash for i32 {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.to_le().hash(state);
        }
    }

    let mut set: HashSet<i32, DummyHasher> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert_eq!(set.get(&4), None);
}

#[test]
fn test_get_with_different_borrowed_type() {
    use hashbrown::HashSet;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct DummyHasher(DefaultHasher);

    impl Hash for i32 {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.to_le().hash(state);
        }
    }

    let mut set: HashSet<i32, DummyHasher> = HashSet::new();
    set.insert(10);
    set.insert(20);
    set.insert(30);

    let value: &i32 = &20;
    assert_eq!(set.get(value), Some(&20));
}

