// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<String, u32, CustomHasher> = HashMap::with_hasher(CustomHasher);
    
    let entry_ref = map.entry_ref("poneyland");
    entry_ref.or_insert_with(|| 3);
}

#[test]
fn test_or_insert_with_existing_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<String, u32, CustomHasher> = HashMap::with_hasher(CustomHasher);
    map.insert("poneyland".to_owned(), 3);

    let entry_ref = map.entry_ref("poneyland");
    *entry_ref.or_insert_with(|| 10) *= 2;
}

#[test]
fn test_or_insert_with_different_key() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<String, u32, CustomHasher> = HashMap::with_hasher(CustomHasher);
    
    let entry_ref = map.entry_ref("unicorn");
    entry_ref.or_insert_with(|| 5);
}

#[test]
fn test_or_insert_with_min_value() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<char, u32, CustomHasher> = HashMap::with_hasher(CustomHasher);
    
    let entry_ref = map.entry_ref(&'a');
    entry_ref.or_insert_with(|| 1);
}

#[test]
fn test_or_insert_with_max_value() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<char, u32, CustomHasher> = HashMap::with_hasher(CustomHasher);
    
    let entry_ref = map.entry_ref(&'z');
    entry_ref.or_insert_with(|| 100);
}

