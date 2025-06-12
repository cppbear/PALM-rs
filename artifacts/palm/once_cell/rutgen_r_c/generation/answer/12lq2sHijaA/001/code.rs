// Answer 0

#[test]
fn test_get_or_init_success() {
    struct TestStruct {
        value: i32,
    }
    
    let once_ref: OnceRef<TestStruct> = OnceRef::new();

    let value = TestStruct { value: 42 };
    let result = once_ref.get_or_init(|| &value);
    assert_eq!(result.value, 42);
}

#[test]
fn test_get_or_init_empty() {
    struct TestStruct {
        value: i32,
    }
    
    let once_ref: OnceRef<TestStruct> = OnceRef::new();

    let result = once_ref.get_or_init(|| {
        let value = TestStruct { value: 100 };
        &value
    });
    assert_eq!(result.value, 100);
}

#[should_panic]
#[test]
fn test_get_or_init_panic_on_err() {
    struct ErrStruct;

    let once_ref: OnceRef<ErrStruct> = OnceRef::new();

    // This imitates a function that might return a Result with an Err type.
    let _: &ErrStruct = once_ref.get_or_init(|| {
        // Trigger an error within the closure to fulfill the test condition
        let _: Result<&ErrStruct, ()> = Err(());
        panic!("Should not reach here")
    });
}

#[test]
fn test_get_or_init_multiple_calls() {
    struct ValueStruct {
        value: i32,
    }

    let once_ref: OnceRef<ValueStruct> = OnceRef::new();
    
    let value = ValueStruct { value: 15 };

    let first_call = once_ref.get_or_init(|| &value);
    let second_call = once_ref.get_or_init(|| &ValueStruct { value: 20 });

    assert_eq!(first_call.value, 15);
    assert_eq!(second_call.value, 15); // Ensure same instance is returned
}

