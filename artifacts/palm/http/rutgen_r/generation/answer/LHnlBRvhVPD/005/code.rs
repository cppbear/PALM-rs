// Answer 0

#[test]
fn test_try_append2_vacant_entry() {
    use std::collections::HashMap;
    use std::hash::Hash;

    struct HeaderName(String);
    impl PartialEq<HeaderName> for HeaderName {
        fn eq(&self, other: &HeaderName) -> bool {
            self.0 == other.0
        }
    }

    struct MaxSizeReached;

    struct MyMap<K, T> {
        entries: Vec<T>,
        extra_values: Vec<T>,
        indices: Vec<Pos>,
        _phantom: std::marker::PhantomData<K>,
    }
    
    struct Pos {
        index: usize,
        hash: u64,
    }

    impl<K, T> MyMap<K, T> 
    where 
        K: Hash + Into<HeaderName> + Copy,
    {
        fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {
            // Simulate a successful reservation
            Ok(())
        }

        fn try_insert_entry(&mut self, _hash: u64, _key: HeaderName, value: T) -> Result<(), MaxSizeReached> {
            self.entries.push(value);
            Ok(())
        }

        fn try_append2(&mut self, key: K, value: T) -> Result<bool, MaxSizeReached> {
            self.try_reserve_one()?;

            let probe = 0; // Dummy probe value
            let pos = 0; // Dummy position value
            let hash = 1; // Dummy hash value
            let danger = 0; // Dummy danger value

            self.try_insert_entry(hash, key.into(), value)?;

            self.indices.push(Pos { index: self.entries.len() - 1, hash });
            Ok(false)
        }
    }

    let mut my_map: MyMap<HeaderName, i32> = MyMap {
        entries: vec![],
        extra_values: vec![],
        indices: vec![],
        _phantom: std::marker::PhantomData,
    };

    let key = HeaderName("my-key".to_string());
    let value = 42;

    let result = my_map.try_append2(key, value);
    assert_eq!(result, Ok(false));
}

#[test]
fn test_try_append2_occupied_entry() {
    use std::collections::HashMap;
    use std::hash::Hash;

    struct HeaderName(String);
    impl PartialEq<HeaderName> for HeaderName {
        fn eq(&self, other: &HeaderName) -> bool {
            self.0 == other.0
        }
    }

    struct MaxSizeReached;

    struct MyMap<K, T> {
        entries: Vec<T>,
        extra_values: Vec<T>,
        indices: Vec<Pos>,
        _phantom: std::marker::PhantomData<K>,
    }
    
    struct Pos {
        index: usize,
        hash: u64,
    }

    impl<K, T> MyMap<K, T> 
    where 
        K: Hash + Into<HeaderName> + Copy,
    {
        fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {
            // Simulate a successful reservation
            Ok(())
        }

        fn try_insert_entry(&mut self, _hash: u64, _key: HeaderName, value: T) -> Result<(), MaxSizeReached> {
            self.entries.push(value);
            Ok(())
        }

        fn append_value(&mut self, _pos: usize, entry: &mut T, _extra_values: &mut Vec<T>, _value: T) {
            // Simulate appending value to occupied entry
        }

        fn try_append2(&mut self, key: K, value: T) -> Result<bool, MaxSizeReached> {
            self.try_reserve_one()?;

            let probe = 0; // Dummy probe value
            let pos = 0; // Dummy position value
            let hash = 1; // Dummy hash value
            let danger = 0; // Dummy danger value

            self.try_insert_entry(hash, key.into(), value)?;
            self.indices.push(Pos { index: pos, hash });

            // Simulating occupied case
            self.append_value(pos, &mut self.entries[pos], &mut self.extra_values, value);
            Ok(true)
        }
    }

    let mut my_map: MyMap<HeaderName, i32> = MyMap {
        entries: vec![42],
        extra_values: vec![],
        indices: vec![Pos { index: 0, hash: 1 }],
        _phantom: std::marker::PhantomData,
    };

    let key = HeaderName("my-key".to_string());
    let value = 24;

    let result = my_map.try_append2(key, value);
    assert_eq!(result, Ok(true));
}

