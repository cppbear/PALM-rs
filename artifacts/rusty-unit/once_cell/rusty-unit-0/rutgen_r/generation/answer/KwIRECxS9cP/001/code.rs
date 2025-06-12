// Answer 0

#[test]
fn test_get_or_try_init_with_existing_value() {
    use std::sync::OnceLock;

    struct MyData {
        value: i32,
    }

    impl MyData {
        fn new(value: i32) -> MyData {
            MyData { value }
        }
    }

    let cell: OnceLock<MyData> = OnceLock::new();
    
    let _ = cell.get_or_init(|| Ok(Box::new(MyData::new(42))));

    assert_eq!(cell.get().map(|val| val.value), Some(42));
}

#[test]
fn test_get_or_try_init_with_multiple_threads() {
    use std::sync::OnceLock;
    use std::thread;

    struct MyData {
        value: i32,
    }

    impl MyData {
        fn new(value: i32) -> MyData {
            MyData { value }
        }
    }

    let cell: OnceLock<MyData> = OnceLock::new();
    
    let handles: Vec<_> = (0..5).map(|_| {
        thread::spawn({
            let cell = &cell;
            move || {
                cell.get_or_init(|| Ok(Box::new(MyData::new(42))))
            }
        })
    }).collect();

    for handle in handles {
        let result = handle.join().unwrap();
        assert!(result.is_ok());
        assert_eq!(cell.get().map(|val| val.value), Some(42));
    }
}

