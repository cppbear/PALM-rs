// Answer 0

#[test]
fn test_ref_mut() {
    struct TestBucket {
        key: i32,
        value: String,
    }

    let mut bucket = Bucket {
        hash: HashValue(123),
        key: 42,
        value: String::from("test"),
    };

    let (key_ref, value_mut_ref) = bucket.ref_mut();

    assert_eq!(*key_ref, 42);
    assert_eq!(*value_mut_ref, "test");

    *value_mut_ref = String::from("updated");
    assert_eq!(bucket.value, "updated");
}

