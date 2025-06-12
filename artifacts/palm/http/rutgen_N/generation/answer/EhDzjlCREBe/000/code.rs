// Answer 0

#[derive(Hash)]
struct TestData {
    data: String,
}

impl TestData {
    fn new(data: &str) -> Self {
        TestData {
            data: data.to_string(),
        }
    }
}

#[test]
fn test_hash_function() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let test_data = TestData::new("example");

    let mut hasher = DefaultHasher::new();
    test_data.hash(&mut hasher);
    let result = hasher.finish();

    assert!(result > 0); // Ensuring that the hash is non-zero, which checks some behavior of the hashing function.
}

#[test]
fn test_hash_function_empty() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let test_data = TestData::new("");

    let mut hasher = DefaultHasher::new();
    test_data.hash(&mut hasher);
    let result = hasher.finish();

    assert!(result > 0); // Confirming that even an empty string gives a valid hash (non-zero).
}

