// Answer 0

#[test]
fn test_muts() {
    struct TestStruct {
        key: i32,
        value: String,
    }

    let mut test_instance = TestStruct {
        key: 42,
        value: String::from("Hello"),
    };

    let (key_ref, value_ref) = test_instance.muts();

    assert_eq!(*key_ref, 42);
    assert_eq!(*value_ref, "Hello");
}

#[test]
fn test_muts_with_negative_key() {
    struct TestStruct {
        key: i32,
        value: String,
    }

    let mut test_instance = TestStruct {
        key: -1,
        value: String::from("Negative key"),
    };

    let (key_ref, value_ref) = test_instance.muts();

    assert_eq!(*key_ref, -1);
    assert_eq!(*value_ref, "Negative key");
}

#[test]
fn test_muts_with_empty_value() {
    struct TestStruct {
        key: i32,
        value: String,
    }

    let mut test_instance = TestStruct {
        key: 0,
        value: String::new(),
    };

    let (key_ref, value_ref) = test_instance.muts();

    assert_eq!(*key_ref, 0);
    assert_eq!(*value_ref, "");
}

#[should_panic]
fn test_muts_panic() {
    struct TestStruct {
        key: i32,
        value: String,
    }

    let mut test_instance = TestStruct {
        key: 10,
        value: String::from("Panic should not occur"),
    };

    // This test function purposely does not call `muts()` correctly,
    // instead, it will be omitted to induce a panic at usage.
    let _unused = test_instance.key; // this should trigger panic on future mutable access.
}

