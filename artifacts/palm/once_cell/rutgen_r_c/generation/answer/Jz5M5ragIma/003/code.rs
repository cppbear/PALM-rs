// Answer 0

#[test]
fn test_wait_not_initialized() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    struct OnceCellForTest(Mutex<Option<u32>>);

    impl OnceCellForTest {
        pub fn new() -> Self {
            OnceCellForTest(Mutex::new(None))
        }

        pub fn is_initialized(&self) -> bool {
            self.0.lock().unwrap().is_some()
        }

        pub fn set(&self, value: u32) {
            *self.0.lock().unwrap() = Some(value);
        }
        
        pub fn wait(&self) -> &u32 {
            while !self.is_initialized() {
                thread::sleep(std::time::Duration::from_millis(1));
            }
            self.get_unchecked()
        }

        pub fn get_unchecked(&self) -> &u32 {
            self.0.lock().unwrap().as_ref().unwrap()
        }
    }

    let cell = Arc::new(OnceCellForTest::new());
    let cell_clone = Arc::clone(&cell);

    let handle = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(50));
        cell_clone.set(42);
    });

    let value = cell.wait();
    assert_eq!(*value, 42);

    handle.join().unwrap();
}

#[test]
fn test_wait_initialized() {
    use std::sync::{Arc, Mutex};

    struct OnceCellForTest(Mutex<Option<u32>>);

    impl OnceCellForTest {
        pub fn new() -> Self {
            OnceCellForTest(Mutex::new(Some(100)))
        }

        pub fn is_initialized(&self) -> bool {
            self.0.lock().unwrap().is_some()
        }
        
        pub fn wait(&self) -> &u32 {
            if !self.is_initialized() {
                panic!("Cell is not initialized");
            }
            self.get_unchecked()
        }

        pub fn get_unchecked(&self) -> &u32 {
            self.0.lock().unwrap().as_ref().unwrap()
        }
    }

    let cell = Arc::new(OnceCellForTest::new());
    let value = cell.wait();
    assert_eq!(*value, 100);
}

