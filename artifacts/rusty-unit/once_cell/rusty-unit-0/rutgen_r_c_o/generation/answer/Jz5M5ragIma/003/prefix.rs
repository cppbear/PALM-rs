// Answer 0

#[test]
fn test_wait_when_not_initialized() {
    let cell = OnceCell::new();
    let value = cell.get();
    let _ = cell.wait(); // This will block until initialized.
}

#[test]
fn test_wait_when_initialized() {
    let cell = OnceCell::new();
    let _ = cell.set(42);
    let value: &u32 = cell.wait();
}

#[test]
fn test_wait_with_thread_initialization() {
    let cell = std::sync::Arc::new(OnceCell::new());
    let t = std::thread::spawn({
        let cell = std::sync::Arc::clone(&cell);
        move || cell.set(100).unwrap()
    });
    let value: &u32 = cell.wait();
    t.join().unwrap();
}

#[test]
#[should_panic]
fn test_wait_uninitialized_panic() {
    let cell = OnceCell::new();
    let _value: &u32 = unsafe { cell.get_unchecked() }; // This should panic as the cell is uninitialized.
}

