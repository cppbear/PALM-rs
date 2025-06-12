// Answer 0

#[test]
fn test_remove_entry_existing_key_preserve_order() {
    use std::collections::HashMap;
    use std::borrow::Borrow;
    use serde_json::Value;

    #[derive(Default)]
    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + std::hash::Hash,
        {
            #[cfg(feature = "preserve_order")]
            return self.swap_remove_entry(key);
            #[cfg(not(feature = "preserve_order"))]
            return self.map.remove_entry(key);
        }

        #[cfg(feature = "preserve_order")]
        pub fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + std::hash::Hash,
        {
            self.map.remove_entry(key)
        }
    }

    let mut my_map = MyMap::default();
    let key = String::from("key1");
    let value = Value::from("value1");
    my_map.map.insert(key.clone(), value.clone());

    let result = my_map.remove_entry(&key);
    assert_eq!(result, Some((key.clone(), value.clone())));
    assert!(my_map.map.get(&key).is_none());
}

#[test]
fn test_remove_entry_non_existing_key() {
    use std::collections::HashMap;
    use std::borrow::Borrow;
    use serde_json::Value;

    #[derive(Default)]
    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + std::hash::Hash,
        {
            #[cfg(feature = "preserve_order")]
            return self.swap_remove_entry(key);
            #[cfg(not(feature = "preserve_order"))]
            return self.map.remove_entry(key);
        }

        #[cfg(feature = "preserve_order")]
        pub fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + std::hash::Hash,
        {
            self.map.remove_entry(key)
        }
    }

    let mut my_map = MyMap::default();
    let key = String::from("key1");
    
    let result = my_map.remove_entry(&key);
    assert_eq!(result, None);
}

