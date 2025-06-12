// Answer 0

#[test]
fn test_get_or_insert_default_with_i32() {
    let mut ext = http::Extensions::new();
    *ext.get_or_insert_default::<i32>() += 2;
    assert_eq!(*ext.get::<i32>().unwrap(), 2);
}

#[test]
fn test_get_or_insert_default_with_string() {
    let mut ext = http::Extensions::new();
    ext.get_or_insert_default::<String>().push_str("Hello");
    assert_eq!(*ext.get::<String>().unwrap(), "Hello");
}

#[test]
fn test_get_or_insert_default_with_vec() {
    let mut ext = http::Extensions::new();
    ext.get_or_insert_default::<Vec<i32>>().push(1);
    assert_eq!(*ext.get::<Vec<i32>>().unwrap(), vec![1]);
}

#[test]
fn test_get_or_insert_default_with_empty_vector() {
    let mut ext = http::Extensions::new();
    let vec_ref = ext.get_or_insert_default::<Vec<u32>>();
    assert!(vec_ref.is_empty());
}

#[test]
fn test_get_or_insert_default_with_boolean() {
    let mut ext = http::Extensions::new();
    *ext.get_or_insert_default::<bool>() = true;
    assert_eq!(*ext.get::<bool>().unwrap(), true);
}

#[test]
#[should_panic]
fn test_get_or_insert_default_with_send_sync() {
    use std::sync::{Arc, Mutex};

    struct NonSync;
    impl Default for NonSync {
        fn default() -> Self {
            NonSync
        }
    }

    let mut ext = http::Extensions::new();
    ext.get_or_insert_default::<Arc<Mutex<NonSync>>>();
}

