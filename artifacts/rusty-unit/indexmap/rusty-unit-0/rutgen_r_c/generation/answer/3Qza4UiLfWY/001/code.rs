// Answer 0

#[test]
fn test_index_mut_existing_key() {
    struct TestMap {
        data: std::collections::HashMap<i32, i32>,
    }
    
    impl MutableKeys for TestMap {
        type Key = i32;
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key>,
        {
            if let Some(value) = self.data.get_mut(key) {
                let index = self.data.keys().position(|k| k == key).unwrap();
                Some((index, &mut key, value))
            } else {
                None
            }
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            let keys: Vec<&i32> = self.data.keys().collect();
            if index < keys.len() {
                let key = keys[index];
                let value = self.data.get_mut(key).unwrap();
                Some((key, value))
            } else {
                None
            }
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!()
        }

        fn retain2<F>(&mut self, keep: F) 
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
            unimplemented!()
        }
    }

    let mut map = TestMap {
        data: std::collections::HashMap::new(),
    };
    map.data.insert(1, 10);
    map.data.insert(2, 20);

    let value = map.index_mut(&1);
    *value += 5;

    assert_eq!(map.data[&1], 15);
}

#[should_panic(expected = "no entry found for key")]
#[test]
fn test_index_mut_non_existing_key() {
    struct TestMap {
        data: std::collections::HashMap<i32, i32>,
    }
    
    impl MutableKeys for TestMap {
        type Key = i32;
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key>,
        {
            if let Some(value) = self.data.get_mut(key) {
                let index = self.data.keys().position(|k| k == key).unwrap();
                Some((index, &mut key, value))
            } else {
                None
            }
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            let keys: Vec<&i32> = self.data.keys().collect();
            if index < keys.len() {
                let key = keys[index];
                let value = self.data.get_mut(key).unwrap();
                Some((key, value))
            } else {
                None
            }
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!()
        }

        fn retain2<F>(&mut self, keep: F) 
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
            unimplemented!()
        }
    }

    let mut map = TestMap {
        data: std::collections::HashMap::new(),
    };
    
    map.index_mut(&3); // this will panic as key 3 does not exist
}

