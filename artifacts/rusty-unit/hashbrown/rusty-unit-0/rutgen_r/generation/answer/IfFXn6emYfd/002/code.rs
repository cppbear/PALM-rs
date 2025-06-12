// Answer 0

#[test]
fn test_get_or_insert_with_success() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};

    struct MySet {
        map: hashbrown::HashMap<String, ()>,
        hash_builder: RandomState,
    }

    impl MySet {
        fn new() -> Self {
            Self {
                map: hashbrown::HashMap::new(),
                hash_builder: RandomState::new(),
            }
        }

        fn get_or_insert_with<Q, F>(&mut self, value: &Q, f: F) -> &String
        where
            Q: Hash + std::cmp::Eq + ?Sized,
            F: FnOnce(&Q) -> String,
        {
            let hash = self.hash_builder.build_hasher().finish();
            let bucket = match self.map.get(value) {
                Some(_) => return self.map.get(value).unwrap(),
                None => {
                    let new = f(value);
                    self.map.insert(new.clone(), ());
                    &new
                }
            };
            bucket
        }
    }

    let mut set = MySet::new();
    let value1 = "cat";
    let value2 = "dog";
    
    let res1 = set.get_or_insert_with(value1, |&v| v.to_owned());
    assert_eq!(res1, "cat");

    let res2 = set.get_or_insert_with(value2, |&v| v.to_owned());
    assert_eq!(res2, "dog");

    let value3 = "fish";
    let res3 = set.get_or_insert_with(value3, |&v| v.to_owned());
    assert_eq!(res3, "fish");
}

#[test]
#[should_panic]
fn test_get_or_insert_with_panic() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};

    struct MySet {
        map: hashbrown::HashMap<String, ()>,
        hash_builder: RandomState,
    }

    impl MySet {
        fn new() -> Self {
            Self {
                map: hashbrown::HashMap::new(),
                hash_builder: RandomState::new(),
            }
        }

        fn get_or_insert_with<Q, F>(&mut self, value: &Q, f: F) -> &String
        where
            Q: Hash + std::cmp::Eq + ?Sized,
            F: FnOnce(&Q) -> String,
        {
            let hash = self.hash_builder.build_hasher().finish();
            let bucket = match self.map.get(value) {
                Some(_) => return self.map.get(value).unwrap(),
                None => {
                    let new = f(value);
                    assert!(value != &new, "new value is not equivalent");
                    self.map.insert(new.clone(), ());
                    &new
                }
            };
            bucket
        }
    }

    let mut set = MySet::new();
    let value = "rust";
    set.get_or_insert_with(value, |_| String::new());
}

