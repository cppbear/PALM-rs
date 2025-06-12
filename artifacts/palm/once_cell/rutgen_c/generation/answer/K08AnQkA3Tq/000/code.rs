// Answer 0

#[test]
fn test_lazy_new() {
    struct TestInit;

    impl TestInit {
        fn init() -> String {
            "Hello, World!".to_string()
        }
    }

    let lazy: Lazy<String, fn() -> String> = Lazy::new(TestInit::init);
    assert_eq!(&*lazy, "HELLO, WORLD!");
}

#[test]
fn test_once_cell_new() {
    let once_cell: OnceCell<i32> = OnceCell::new();
    assert_eq!(once_cell.get(), None);
}

#[test]
fn test_once_cell_set_and_get() {
    let once_cell: OnceCell<i32> = OnceCell::new();
    assert!(once_cell.set(42).is_ok());
    assert_eq!(once_cell.get(), Some(&42));
}

#[test]
fn test_once_cell_take() {
    let mut once_cell: OnceCell<i32> = OnceCell::new();
    once_cell.set(42).unwrap();
    assert_eq!(once_cell.take(), Some(42));
    assert_eq!(once_cell.take(), None);
}

#[test]
fn test_once_cell_try_insert() {
    let once_cell: OnceCell<i32> = OnceCell::new();
    assert!(once_cell.try_insert(42).is_ok());
    assert_eq!(once_cell.try_insert(43), Err((&42, 43)));
}

#[test]
fn test_once_cell_get_or_init() {
    let once_cell: OnceCell<i32> = OnceCell::new();
    let value = once_cell.get_or_init(|| 50);
    assert_eq!(*value, 50);
}

