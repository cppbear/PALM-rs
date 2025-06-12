// Answer 0

#[test]
fn test_fmt_debug_empty() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let index_map: IndexMap<i32, i32, DummyHasher> = IndexMap::with_capacity_and_hasher(0, DummyHasher);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", index_map);
    assert!(result.is_ok());
    assert_eq!(output, "{}");
}

#[test]
fn test_fmt_debug_non_empty() {
    use std::collections::hash_map::RandomState;

    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    index_map.insert(1, 10);
    index_map.insert(2, 20);

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", index_map);
    assert!(result.is_ok());
    assert!(output.contains("1: 10"));
    assert!(output.contains("2: 20"));
}

#[test]
#[should_panic]
fn test_fmt_debug_panic_on_invalid_format() {
    struct Invalid;

    impl fmt::Debug for Invalid {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("invalid format")
        }
    }

    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let index_map: IndexMap<Invalid, Invalid, DummyHasher> = IndexMap::with_capacity_and_hasher(0, DummyHasher);
    let _ = write!(String::new(), "{:?}", index_map);
}

