// Answer 0

#[test]
fn test_try_insert_entry_max_size_reached() {
    const MAX_SIZE: usize = 5;

    struct HashValue(u64);
    struct HeaderName(String);
    struct Bucket<T> {
        hash: HashValue,
        key: HeaderName,
        value: T,
        links: Option<Box<Bucket<T>>>,
    }
    
    struct Map<T> {
        entries: Vec<Bucket<T>>,
    }
    
    impl<T> Map<T> {
        fn try_insert_entry(
            &mut self,
            hash: HashValue,
            key: HeaderName,
            value: T,
        ) -> Result<(), MaxSizeReached> {
            if self.entries.len() >= MAX_SIZE {
                return Err(MaxSizeReached::new());
            }

            self.entries.push(Bucket {
                hash,
                key,
                value,
                links: None,
            });

            Ok(())
        }
    }
    
    struct MaxSizeReached;

    impl MaxSizeReached {
        fn new() -> Self {
            MaxSizeReached
        }
    }

    let mut map = Map::<i32> {
        entries: vec![
            Bucket { hash: HashValue(1), key: HeaderName("Key1".to_string()), value: 1, links: None },
            Bucket { hash: HashValue(2), key: HeaderName("Key2".to_string()), value: 2, links: None },
            Bucket { hash: HashValue(3), key: HeaderName("Key3".to_string()), value: 3, links: None },
            Bucket { hash: HashValue(4), key: HeaderName("Key4".to_string()), value: 4, links: None },
            Bucket { hash: HashValue(5), key: HeaderName("Key5".to_string()), value: 5, links: None },
        ],
    };

    let result = map.try_insert_entry(HashValue(6), HeaderName("Key6".to_string()), 6);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MaxSizeReached::new());
}

