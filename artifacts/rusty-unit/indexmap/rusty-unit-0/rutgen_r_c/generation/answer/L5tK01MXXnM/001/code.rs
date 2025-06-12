// Answer 0

#[test]
fn test_from_key_existing_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use crate::{IndexMap, Equivalent};

    struct MyKey(i32);
    struct MyValue(String);

    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<MyKey> for MyKey {
        fn equivalent(&self, other: &MyKey) -> bool {
            self.0 == other.0
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(10, DefaultHasher::new());
    map.insert(MyKey(1), MyValue("value_1".to_string()));

    let builder = RawEntryBuilder { map: &map };
    let result = builder.from_key(&MyKey(1));

    assert_eq!(result, Some((&MyKey(1), &MyValue("value_1".to_string()))));
}

#[test]
fn test_from_key_non_existing_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use crate::{IndexMap, Equivalent};

    struct MyKey(i32);
    struct MyValue(String);

    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<MyKey> for MyKey {
        fn equivalent(&self, other: &MyKey) -> bool {
            self.0 == other.0
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(10, DefaultHasher::new());
    map.insert(MyKey(1), MyValue("value_1".to_string()));

    let builder = RawEntryBuilder { map: &map };
    let result = builder.from_key(&MyKey(2));

    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_from_key_panic_conditions() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use crate::{IndexMap, Equivalent};

    struct MyKey(i32);
    struct MyValue(String);

    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<MyKey> for MyKey {
        fn equivalent(&self, other: &MyKey) -> bool {
            self.0 == other.0
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(10, DefaultHasher::new());
    // Intentionally not inserting the key to trigger panic scenario in actual use
    let builder = RawEntryBuilder { map: &map };
    let _result = builder.from_key(&MyKey(1));
}

