// Answer 0

#[test]
fn test_set_returns_err_when_cell_is_full() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestCell {
        value: AtomicUsize,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell {
                value: AtomicUsize::new(0),
            }
        }

        fn compare_exchange(&self, new_value: NonZeroUsize) -> Result<(), ()> {
            let current = self.value.load(Ordering::SeqCst);
            if current == 0 {
                self.value.store(new_value.get(), Ordering::SeqCst);
                Ok(())
            } else {
                Err(())
            }
        }

        pub fn set(&self, value: NonZeroUsize) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let cell = TestCell::new();
    let value_full = NonZeroUsize::new(1).unwrap();
    cell.set(value_full).unwrap(); // Initial set to fill the cell

    let value_to_set = NonZeroUsize::new(2).unwrap();
    let result = cell.set(value_to_set);
    assert_eq!(result, Err(()));
}

#[test]
fn test_set_does_not_panic_on_full_cell() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestCell {
        value: AtomicUsize,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell {
                value: AtomicUsize::new(1), // Pre-fill to trigger the full scenario
            }
        }

        fn compare_exchange(&self, new_value: NonZeroUsize) -> Result<(), ()> {
            let current = self.value.load(Ordering::SeqCst);
            if current == 0 {
                self.value.store(new_value.get(), Ordering::SeqCst);
                Ok(())
            } else {
                Err(())
            }
        }

        pub fn set(&self, value: NonZeroUsize) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let cell = TestCell::new();
    let value_to_set = NonZeroUsize::new(2).unwrap();
    let result = cell.set(value_to_set);
    assert_eq!(result, Err(()));
}

