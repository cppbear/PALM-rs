// Answer 0

#[test]
fn test_union_debug_with_unique_elements() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;
    use std::fmt;

    struct CustomHasher(BuildHasher);

    impl BuildHasher for CustomHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let hasher = CustomHasher(DefaultHasher);
    let mut set_a: IndexSet<i32, CustomHasher> = IndexSet::with_hasher(hasher);
    let mut set_b: IndexSet<i32, CustomHasher> = IndexSet::with_hasher(hasher);
    
    for i in 1..=1000 {
        set_a.insert(i);
        set_b.insert(i + 1000);
    }

    let union = Union {
        iter: set_a.iter().chain(set_b.difference(&set_a)),
    };

    let mut formatter = fmt::Formatter::new();
    union.fmt(&mut formatter);
}

#[test]
fn test_union_debug_with_empty_set() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;
    use std::fmt;

    struct CustomHasher(BuildHasher);

    impl BuildHasher for CustomHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let hasher = CustomHasher(DefaultHasher);
    let set_a: IndexSet<i32, CustomHasher> = IndexSet::with_hasher(hasher);
    let set_b: IndexSet<i32, CustomHasher> = IndexSet::with_hasher(hasher);

    let union = Union {
        iter: set_a.iter().chain(set_b.difference(&set_a)),
    };

    let mut formatter = fmt::Formatter::new();
    union.fmt(&mut formatter);
}

#[test]
fn test_union_debug_with_some_common_elements() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;
    use std::fmt;

    struct CustomHasher(BuildHasher);

    impl BuildHasher for CustomHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let hasher = CustomHasher(DefaultHasher);
    let mut set_a: IndexSet<i32, CustomHasher> = IndexSet::with_hasher(hasher);
    let mut set_b: IndexSet<i32, CustomHasher> = IndexSet::with_hasher(hasher);
    
    for i in 1..=500 {
        set_a.insert(i);
    }
    
    for i in 250..=750 {
        set_b.insert(i);
    }

    let union = Union {
        iter: set_a.iter().chain(set_b.difference(&set_a)),
    };

    let mut formatter = fmt::Formatter::new();
    union.fmt(&mut formatter);
}

#[test]
fn test_union_debug_with_reversed_sets() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;
    use std::fmt;

    struct CustomHasher(BuildHasher);

    impl BuildHasher for CustomHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let hasher = CustomHasher(DefaultHasher);
    let mut set_a: IndexSet<i32, CustomHasher> = IndexSet::with_hasher(hasher);
    let mut set_b: IndexSet<i32, CustomHasher> = IndexSet::with_hasher(hasher);
    
    for i in 1000..=2000 {
        set_a.insert(i);
    }
    
    for i in 500..=1500 {
        set_b.insert(i);
    }

    let union = Union {
        iter: set_a.iter().chain(set_b.difference(&set_a)),
    };

    let mut formatter = fmt::Formatter::new();
    union.fmt(&mut formatter);
}

