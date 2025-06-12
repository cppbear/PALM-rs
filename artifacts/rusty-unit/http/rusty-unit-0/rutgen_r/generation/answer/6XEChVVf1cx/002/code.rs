// Answer 0

#[test]
fn test_try_insert_entry_success() {
    const MAX_SIZE: usize = 5;

    struct HashValue(u64);
    struct HeaderName(String);
    struct MaxSizeReached;

    struct Bucket<T> {
        hash: HashValue,
        key: HeaderName,
        value: T,
        links: Option<()>,
    }

    struct HeaderMap<T> {
        entries: Vec<Bucket<T>>,
    }

    impl<T> HeaderMap<T> {
        fn new() -> Self {
            HeaderMap {
                entries: Vec::new(),
            }
        }

        fn try_insert_entry(
            &mut self,
            hash: HashValue,
            key: HeaderName,
            value: T,
        ) -> Result<(), MaxSizeReached> {
            if self.entries.len() >= MAX_SIZE {
                return Err(MaxSizeReached);
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

    let mut header_map: HeaderMap<i32> = HeaderMap::new();

    // Test inserting an entry when the size is below MAX_SIZE
    let key = HeaderName("Content-Type".to_string());
    let hash = HashValue(12345);
    let value = 200;
    
    let result = header_map.try_insert_entry(hash, key, value);
    assert_eq!(result, Ok(()));
}

