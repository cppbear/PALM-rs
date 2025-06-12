// Answer 0

#[test]
fn test_get_mut_with_string() {
    let mut ext = Extensions::new();
    ext.insert(String::from("Hello"));
    if let Some(s) = ext.get_mut::<String>() {
        s.push_str(" World");
    }
}

#[test]
fn test_get_mut_with_integer() {
    let mut ext = Extensions::new();
    ext.insert(42);
    if let Some(i) = ext.get_mut::<i32>() {
        *i += 10;
    }
}

#[test]
fn test_get_mut_on_empty_extensions() {
    let mut ext = Extensions::new();
    let result: Option<&mut String> = ext.get_mut();
    assert!(result.is_none());
}

#[test]
fn test_get_mut_with_large_string() {
    let mut ext = Extensions::new();
    ext.insert(String::from("A long string that is well within the allowed length of 100 characters."));
    if let Some(s) = ext.get_mut::<String>() {
        s.push_str(" Additional text.");
    }
}

#[test]
fn test_get_mut_with_multiple_insertions() {
    let mut ext = Extensions::new();
    ext.insert(String::from("First"));
    ext.insert(11);
    if let Some(s) = ext.get_mut::<String>() {
        s.push_str(" Second");
    }
    if let Some(i) = ext.get_mut::<i32>() {
        *i += 1;
    }
}

#[test]
fn test_get_mut_panic_condition() {
    let mut ext = Extensions::new();
    ext.insert(String::from("Test"));
    let mut result = ext.get_mut::<u32>();
    assert!(result.is_none());
}

#[test]
fn test_get_mut_thread_safety() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let ext = Arc::new(Mutex::new(Extensions::new()));
    ext.lock().unwrap().insert(1);

    let mut handles = vec![];

    for _ in 0..10 {
        let ext_clone = Arc::clone(&ext);
        let handle = thread::spawn(move || {
            let mut ext = ext_clone.lock().unwrap();
            if let Some(val) = ext.get_mut::<i32>() {
                *val += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

