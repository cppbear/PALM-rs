// Answer 0

#[test]
fn test_lazy_debug_format() {
    struct TestValue;

    impl fmt::Debug for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("TestValue")
        }
    }

    let once_cell = OnceCell(TestValue);
    let lazy = Lazy {
        cell: once_cell,
        init: Cell::new(None),
    };

    let mut buffer = Vec::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut buffer);
        lazy.fmt(formatter).unwrap();
    }

    let result = String::from_utf8(buffer).unwrap();
    assert!(result.contains("Lazy"));
    assert!(result.contains("TestValue"));
    assert!(result.contains("init"));
}

