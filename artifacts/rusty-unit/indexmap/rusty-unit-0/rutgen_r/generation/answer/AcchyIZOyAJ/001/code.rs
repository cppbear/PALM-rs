// Answer 0

#[test]
fn test_key_mut() {
    struct TestStruct<K> {
        key: K,
    }

    let mut entry = TestStruct { key: 42 }; // Initialize with an integer key

    let key_mut = entry.key_mut(); // Call the function under test
    *key_mut += 1; // Modify the key to check mutability

    assert_eq!(entry.key, 43); // Assert the expected outcome after modification
}

#[test]
fn test_key_mut_with_string() {
    struct TestStruct<K> {
        key: K,
    }

    let mut entry = TestStruct { key: String::from("initial") }; // Initialize with a string key

    let key_mut = entry.key_mut(); // Call the function under test
    key_mut.push_str(" appended"); // Modify the key to check mutability

    assert_eq!(entry.key, "initial appended"); // Assert the expected outcome after modification
}

#[test]
#[should_panic]
fn test_key_mut_panic_on_double_borrow() {
    struct TestStruct<K> {
        key: K,
    }

    let mut entry = TestStruct { key: 100 }; // Initialize with an integer key
    let first_borrow = entry.key_mut(); // First mutable borrow

    // Simulate a panic condition by trying to borrow again
    let _second_borrow = entry.key_mut(); // This should panic

    drop(first_borrow); // Clean up the first mutable borrow
}

