// Answer 0

#[test]
fn test_set_full_state() {
    struct TestStruct {
        val: usize,
    }

    let mut test_value = TestStruct { val: 42 };
    let value_ref: &TestStruct = &test_value;

    let once_ref = OnceRef::new();
    once_ref.inner.store(value_ref as *const _ as *mut _, Ordering::Release);

    let result = once_ref.set(value_ref);
}

#[test]
fn test_set_with_different_value() {
    struct TestStruct {
        val: usize,
    }

    let mut existing_value = TestStruct { val: 42 };
    let existing_ref: &TestStruct = &existing_value;

    let mut new_value = TestStruct { val: 100 };
    let new_ref: &TestStruct = &new_value;

    let once_ref = OnceRef::new();
    once_ref.inner.store(existing_ref as *const _ as *mut _, Ordering::Release);

    let result = once_ref.set(new_ref);
}

#[test]
fn test_set_on_full_with_different_memory_address() {
    struct TestStruct {
        val: usize,
    }

    let existing_value = TestStruct { val: 42 };
    let existing_ref: &TestStruct = &existing_value;

    let other_value = TestStruct { val: 100 };
    let other_ref: &TestStruct = &other_value;

    let once_ref = OnceRef::new();
    once_ref.inner.store(existing_ref as *const _ as *mut _, Ordering::Release);

    let result = once_ref.set(other_ref);
}

