// Answer 0

#[test]
fn test_deref_mut() {
    use crate::once_cell::Lazy;

    struct TestStruct {
        value: i32,
    }

    let mut lazy_instance = Lazy::new(|| TestStruct { value: 10 });

    // Calling deref_mut to get a mutable reference to the underlying value
    {
        let value_ref: &mut TestStruct = lazy_instance.deref_mut();
        // Modify the value to test the mutable reference
        value_ref.value += 5;
        assert_eq!(value_ref.value, 15);
    }

    // Ensure the original value has been updated
    {
        let value_ref: &mut TestStruct = lazy_instance.deref_mut();
        assert_eq!(value_ref.value, 15);
    }
}

#[test]
#[should_panic]
fn test_deref_mut_panic() {
    use crate::once_cell::Lazy;

    struct TestStruct {
        value: i32,
    }

    let lazy_instance: Lazy<TestStruct> = Lazy::new(|| panic!("Panic triggered")); // create a Lazy instance that will panic

    // This will cause a panic when deref_mut is called
    let _ = lazy_instance.deref_mut();
}

