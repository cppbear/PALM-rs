// Answer 0

#[test]
fn test_make_hasher() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::hash::BuildHasher;

    struct MyBuildHasher;

    impl BuildHasher for MyBuildHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let hasher = MyBuildHasher;
    let make_hash = make_hasher::<&str, i32, MyBuildHasher>(&hasher);

    let value = ("test", 42);
    let hash_value = make_hash(&value);

    assert!(hash_value > 0);
}

#[test]
fn test_make_hasher_with_different_inputs() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::hash::BuildHasher;

    struct MyBuildHasher;

    impl BuildHasher for MyBuildHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let hasher = MyBuildHasher;
    let make_hash = make_hasher::<&str, i32, MyBuildHasher>(&hasher);

    let value1 = ("test", 42);
    let value2 = ("test", 43);
    let hash_value1 = make_hash(&value1);
    let hash_value2 = make_hash(&value2);

    assert_ne!(hash_value1, hash_value2);
}

