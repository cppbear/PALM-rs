// Answer 0

#[test]
fn test_read_value() {
    struct RawTable<T> {
        value: T,
    }

    impl<T> RawTable<T> {
        fn new(value: T) -> Self {
            RawTable { value }
        }

        fn as_ptr(&self) -> *const T {
            &self.value
        }

        unsafe fn read(&self) -> T {
            self.as_ptr().read()
        }
    }

    let table = RawTable::new(42);
    let value: i32;

    unsafe {
        value = table.read();
    }

    assert_eq!(value, 42);
}

#[test]
#[should_panic]
fn test_read_invalid_memory() {
    struct RawTable<T> {
        value: T,
    }

    impl<T> RawTable<T> {
        fn new(value: T) -> Self {
            RawTable { value }
        }

        fn as_ptr(&self) -> *const T {
            &self.value
        }

        unsafe fn read(&self) -> T {
            self.as_ptr().read()
        }
    }

    let table = RawTable::new(Box::new(42));
    let _value: Box<i32>;

    unsafe {
        let leaked_value = Box::into_raw(Box::new(30));
        _value = table.read(); // Will not panic as Box is valid, but causing leakage
        std::mem::forget(Box::from_raw(leaked_value)); // Prevent double free
    }
}

