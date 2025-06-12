// Answer 0

#[test]
fn test_key_mut() {
    struct TestStruct {
        key: i32,
    }

    let mut test_instance = TestStruct { key: 10 };
    let key_ref = test_instance.key_mut();
    *key_ref = 20;

    assert_eq!(test_instance.key, 20);
}

