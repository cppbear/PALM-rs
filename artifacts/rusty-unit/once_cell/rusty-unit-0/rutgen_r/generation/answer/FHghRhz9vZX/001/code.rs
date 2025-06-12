// Answer 0

#[test]
fn test_with_value() {
    struct OnceCell<T> {
        inner: std::cell::UnsafeCell<Option<T>>,
    }

    // Test with a valid value of type i32
    let cell_i32 = with_value(42);
    unsafe {
        assert_eq!(*(cell_i32.inner.get()), Some(42));
    }

    // Test with a valid value of type String
    let cell_string = with_value(String::from("hello"));
    unsafe {
        assert_eq!(*(cell_string.inner.get()), Some(String::from("hello")));
    }

    // Test with a valid value of type f64
    let cell_f64 = with_value(3.14);
    unsafe {
        assert_eq!(*(cell_f64.inner.get()), Some(3.14));
    }
}

fn with_value<T>(value: T) -> OnceCell<T> {
    OnceCell { inner: std::cell::UnsafeCell::new(Some(value)) }
}

