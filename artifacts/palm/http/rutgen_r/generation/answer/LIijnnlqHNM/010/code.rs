// Answer 0

#[test]
fn test_try_insert2_vacant_entry() {
    use std::collections::HashMap;
    use std::hash::Hash;

    struct HeaderName(String);

    impl PartialEq<HeaderName> for HeaderName {
        fn eq(&self, other: &HeaderName) -> bool {
            self.0 == other.0
        }
    }

    struct MyMap<K, T> {
        entries: Vec<(K, T)>,
        indices: Vec<Option<Pos>>,
        max_size: usize,
    }

    impl<K, T> MyMap<K, T>
    where
        K: Hash + Into<HeaderName> + Clone,
    {
        fn new(max_size: usize) -> Self {
            MyMap {
                entries: Vec::new(),
                indices: Vec::new(),
                max_size,
            }
        }

        fn try_reserve_one(&mut self) -> Result<(), ()> {
            if self.entries.len() < self.max_size {
                self.indices.push(None);
                Ok(())
            } else {
                Err(())
            }
        }

        fn try_insert_entry(&mut self, _hash: usize, key: K, value: T) -> Result<(), ()> {
            self.entries.push((key, value));
            Ok(())
        }

        fn insert_occupied(&self, _pos: usize, value: T) -> Option<T> {
            Some(value)
        }

        fn try_insert2(&mut self, key: K, value: T) -> Result<Option<T>, ()> {
            self.try_reserve_one()?;

            let index = self.entries.len();
            self.try_insert_entry(0, key.clone(), value)?;
            self.indices.push(Some(Pos::new(index, 0)));

            Ok(None)
        }
    }

    #[derive(Clone)]
    struct Pos(usize, usize);

    impl Pos {
        fn new(index: usize, _hash: usize) -> Self {
            Pos(index, 0)
        }
    }

    let mut map = MyMap::new(2);
    let key = HeaderName("key".to_string());
    let value = "value";
    let result = map.try_insert2(key.clone(), value);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[test]
#[should_panic]
fn test_try_insert2_exceeding_max_size() {
    use std::collections::HashMap;
    use std::hash::Hash;

    struct HeaderName(String);

    impl PartialEq<HeaderName> for HeaderName {
        fn eq(&self, other: &HeaderName) -> bool {
            self.0 == other.0
        }
    }

    struct MyMap<K, T> {
        entries: Vec<(K, T)>,
        indices: Vec<Option<Pos>>,
        max_size: usize,
    }

    impl<K, T> MyMap<K, T>
    where
        K: Hash + Into<HeaderName> + Clone,
    {
        fn new(max_size: usize) -> Self {
            MyMap {
                entries: Vec::new(),
                indices: Vec::new(),
                max_size,
            }
        }

        fn try_reserve_one(&mut self) -> Result<(), ()> {
            if self.entries.len() < self.max_size {
                self.indices.push(None);
                Ok(())
            } else {
                Err(())
            }
        }

        fn try_insert_entry(&mut self, _hash: usize, key: K, value: T) -> Result<(), ()> {
            self.entries.push((key, value));
            Ok(())
        }

        fn insert_occupied(&self, _pos: usize, value: T) -> Option<T> {
            Some(value)
        }

        fn try_insert2(&mut self, key: K, value: T) -> Result<Option<T>, ()> {
            self.try_reserve_one()?;
            let index = self.entries.len();
            self.try_insert_entry(0, key.clone(), value)?;
            self.indices.push(Some(Pos::new(index, 0)));

            Ok(None)
        }
    }

    #[derive(Clone)]
    struct Pos(usize, usize);

    impl Pos {
        fn new(index: usize, _hash: usize) -> Self {
            Pos(index, 0)
        }
    }

    let mut map = MyMap::new(1);
    map.try_insert2(HeaderName("key1".to_string()), "value1").unwrap();
    map.try_insert2(HeaderName("key2".to_string()), "value2").unwrap(); // This should panic
}

