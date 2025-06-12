// Answer 0

#[derive(Default)]
struct Lazy<T> {
    value: Option<T>,
}

impl<T> Lazy<T> {
    fn force_mut(&mut self) -> &mut T {
        if self.value.is_none() {
            // Trigger panic condition by attempting to dereference a None
            panic!("Lazy value has not been initialized");
        }
        self.value.as_mut().unwrap()
    }

    fn initialize(&mut self, value: T) {
        self.value = Some(value);
    }
}

#[test]
fn test_deref_mut_initialized() {
    let mut lazy = Lazy::default();
    lazy.initialize(42);
    let value: &mut i32 = lazy.force_mut();
    *value += 1;
    assert_eq!(*value, 43);
}

#[test]
#[should_panic(expected = "Lazy value has not been initialized")]
fn test_deref_mut_uninitialized() {
    let mut lazy: Lazy<i32> = Lazy::default();
    lazy.force_mut(); // This should panic
}

#[test]
fn test_deref_mut_twice() {
    let mut lazy = Lazy::default();
    lazy.initialize(10);
    {
        let value1: &mut i32 = lazy.force_mut();
        *value1 += 5;
        assert_eq!(*value1, 15);
    }
    {
        let value2: &mut i32 = lazy.force_mut();
        *value2 += 10;
        assert_eq!(*value2, 25);
    }
}

