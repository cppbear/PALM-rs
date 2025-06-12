// Answer 0

#[test]
fn test_get_unchecked() {
    struct Dummy {
        value: i32,
    }

    let cell = OnceCell::new();
    
    let init_value = Dummy { value: 42 };
    unsafe {
        cell.set(init_value).expect("Failed to set value");
        let retrieved: &Dummy = cell.get_unchecked();
        assert_eq!(retrieved.value, 42);
    }
}

#[test]
#[should_panic(expected = "cell is not initialized")]
fn test_get_unchecked_uninitialized() {
    let cell: OnceCell<Dummy> = OnceCell::new();
    
    unsafe {
        let _: &Dummy = cell.get_unchecked(); // This should panic
    }
}

#[test]
fn test_get_unchecked_with_multiple_initializations() {
    struct Dummy {
        value: i32,
    }

    let cell = OnceCell::new();
    
    let init_value1 = Dummy { value: 10 };
    let init_value2 = Dummy { value: 20 };
    
    unsafe {
        cell.set(init_value1).expect("Failed to set first value");
        assert_eq!(cell.get_unchecked().value, 10);
        
        // This should not panic as we're using the same instance; hence doesn't trigger any overwrite;
        cell.set(init_value2).expect("Failed to set second value");
        assert_eq!(cell.get_unchecked().value, 20);
    }
}

