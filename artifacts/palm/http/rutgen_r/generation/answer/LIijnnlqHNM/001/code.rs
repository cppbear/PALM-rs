// Answer 0

#[test]
fn test_try_insert2_max_size_reached() {
    use std::hash::Hash;
    use std::collections::HashMap;

    #[derive(Hash, PartialEq)]
    struct HeaderName(String);

    struct MyMap<K, T> {
        entries: Vec<(K, T)>,
        indices: Vec<usize>,
        max_size: usize,
    }

    impl<K: Hash + Into<HeaderName>, T> MyMap<K, T> {
        fn new(max_size: usize) -> Self {
            MyMap {
                entries: Vec::new(),
                indices: Vec::new(),
                max_size,
            }
        }

        fn try_reserve_one(&mut self) -> Result<(), ()> {
            if self.entries.len() >= self.max_size {
                Err(())
            } else {
                Ok(())
            }
        }

        fn try_insert_entry(&mut self, _hash: usize, key: K, value: T) -> Result<(), ()> {
            self.entries.push((key, value));
            Ok(())
        }

        fn try_insert2(&mut self, key: K, value: T) -> Result<Option<T>, ()> {
            self.try_reserve_one()?;

            self.try_insert_entry(0, key, value)?;
            Ok(None)
        }
    }

    let mut my_map = MyMap::new(2);
    my_map.entries.push((HeaderName("Key1".to_string()), 1));
    my_map.indices.push(0);
    
    // Trying to insert the third element should hit the max size reached constraint
    let result = my_map.try_insert2(HeaderName("Key2".to_string()), 2);

    assert!(result.is_err());
}

