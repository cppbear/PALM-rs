// Answer 0

#[test]
fn test_hash_path_and_query() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    #[derive(Clone)]
    struct DummyPathAndQuery {
        data: ByteStr,
        query: u16,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    struct ByteStr {
        bytes: Bytes,
    }

    impl hash::Hash for DummyPathAndQuery {
        fn hash<H: hash::Hasher>(&self, state: &mut H) {
            self.data.hash(state);
        }
    }

    let data1 = ByteStr {
        bytes: Bytes::from_static(b"example1"),
    };
    let data2 = ByteStr {
        bytes: Bytes::from_static(b"example2"),
    };

    let path_query1 = DummyPathAndQuery { data: data1.clone(), query: 0 };
    let path_query2 = DummyPathAndQuery { data: data2.clone(), query: 0 };

    let mut hasher1 = DefaultHasher::new();
    path_query1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    path_query2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_ne!(hash1, hash2); // Ensure that different instances produce different hashes.
}

