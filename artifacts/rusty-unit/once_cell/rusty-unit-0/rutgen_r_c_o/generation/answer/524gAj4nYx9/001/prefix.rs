// Answer 0

#[test]
fn test_get_unchecked_with_i32() {
    let cell = OnceCell::new();
    let _ = cell.set(42).unwrap();
    unsafe {
        let value: &i32 = cell.get_unchecked();
    }
}

#[test]
fn test_get_unchecked_with_string() {
    let cell = OnceCell::new();
    let _ = cell.set(String::from("Hello")).unwrap();
    unsafe {
        let value: &String = cell.get_unchecked();
    }
}

#[test]
fn test_get_unchecked_with_float() {
    let cell = OnceCell::new();
    let _ = cell.set(3.14).unwrap();
    unsafe {
        let value: &f64 = cell.get_unchecked();
    }
}

#[test]
fn test_get_unchecked_after_take() {
    let cell = OnceCell::new();
    let _ = cell.set(100).unwrap();
    let _ = cell.take(); // Take it to make sure it is okay to access after
    unsafe {
        let value: &i32 = cell.get_unchecked();
    }
}

#[test]
#[should_panic]
fn test_get_unchecked_panic_uninitialized() {
    let cell = OnceCell::new();
    unsafe {
        let _value: &i32 = cell.get_unchecked(); // This should panic as cell is uninitialized
    }
}

