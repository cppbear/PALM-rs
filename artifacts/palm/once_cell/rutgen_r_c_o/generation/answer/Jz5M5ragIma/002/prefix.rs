// Answer 0

#[test]
fn test_wait_initialized() {
    let cell = std::sync::Arc::new(OnceCell::new());
    cell.set(42).unwrap();
    let value: &u32 = cell.wait();
}

#[test]
fn test_wait_not_initialized() {
    let cell = std::sync::Arc::new(OnceCell::new());
    let t = std::thread::spawn({
        let cell = std::sync::Arc::clone(&cell);
        move || cell.set(99).unwrap()
    });
    let value: &u32 = cell.wait();
    t.join().unwrap();
} 

#[should_panic]
fn test_wait_uninitialized_panic() {
    let cell = OnceCell::new();
    let _value: &u32 = cell.wait(); // This should panic as the cell is not initialized.
}

