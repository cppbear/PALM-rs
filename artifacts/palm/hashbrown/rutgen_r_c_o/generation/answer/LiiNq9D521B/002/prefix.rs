// Answer 0

#[test]
fn test_entry_occupied_even_hash() {
    struct EvenHashBuilder;

    impl BuildHasher for EvenHashBuilder {
        type Hasher = std::hash::rustc_hash::FxHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    let mut table = HashTable::new_in(EvenHashBuilder);
    let hasher = |val: &u64| {
        let mut hasher = EvenHashBuilder::default().build_hasher();
        hasher.write_u64(*val);
        hasher.finish()
    };

    table.insert_unique(hasher(&2), 2, hasher);
    let entry = table.entry(hasher(&2), |&val| val % 2 == 0, hasher);
}

#[test]
fn test_entry_occupied_large_hash() {
    struct LargeHashBuilder;

    impl BuildHasher for LargeHashBuilder {
        type Hasher = std::hash::rustc_hash::FxHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    let mut table = HashTable::new_in(LargeHashBuilder);
    let hasher = |val: &u64| {
        let mut hasher = LargeHashBuilder::default().build_hasher();
        hasher.write_u64(*val);
        hasher.finish()
    };

    table.insert_unique(hasher(&100), 100, hasher);
    let entry = table.entry(hasher(&100), |&val| val % 2 == 0, hasher);
}

#[test]
fn test_entry_occupied_hash_zero() {
    struct ZeroHashBuilder;

    impl BuildHasher for ZeroHashBuilder {
        type Hasher = std::hash::rustc_hash::FxHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    let mut table = HashTable::new_in(ZeroHashBuilder);
    let hasher = |val: &u64| {
        let mut hasher = ZeroHashBuilder::default().build_hasher();
        hasher.write_u64(*val);
        hasher.finish()
    };

    table.insert_unique(hasher(&0), 0, hasher);
    let entry = table.entry(hasher(&0), |&val| val % 2 == 0, hasher);
}

#[test]
fn test_entry_occupied_non_sequential_hash() {
    struct NonSequentialHashBuilder;

    impl BuildHasher for NonSequentialHashBuilder {
        type Hasher = std::hash::rustc_hash::FxHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    let mut table = HashTable::new_in(NonSequentialHashBuilder);
    let hasher = |val: &u64| {
        let mut hasher = NonSequentialHashBuilder::default().build_hasher();
        hasher.write_u64(*val);
        hasher.finish()
    };

    table.insert_unique(hasher(&5), 5, hasher);
    table.insert_unique(hasher(&15), 15, hasher);
    let entry = table.entry(hasher(&5), |&val| val % 2 == 0, hasher);
}

