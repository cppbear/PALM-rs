// Answer 0

#[test]
fn test_get_present_value() {
    struct MySet {
        map: super::IndexMap<i32, (), std::collections::hash_map::RandomState>,
    }

    impl MySet {
        fn new() -> Self {
            MySet { map: super::IndexMap::new() }
        }
        
        fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }
        
        fn get(&self, value: &i32) -> Option<&i32> {
            self.map.get_key_value(value).map(|(x, &())| x)
        }
    }

    let mut set = MySet::new();
    set.insert(1);
    set.insert(2);
    assert_eq!(set.get(&1), Some(&1));
    assert_eq!(set.get(&2), Some(&2));
}

#[test]
fn test_get_absent_value() {
    struct MySet {
        map: super::IndexMap<i32, (), std::collections::hash_map::RandomState>,
    }

    impl MySet {
        fn new() -> Self {
            MySet { map: super::IndexMap::new() }
        }
        
        fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }
        
        fn get(&self, value: &i32) -> Option<&i32> {
            self.map.get_key_value(value).map(|(x, &())| x)
        }
    }

    let mut set = MySet::new();
    set.insert(1);
    set.insert(2);
    assert_eq!(set.get(&3), None);
}

