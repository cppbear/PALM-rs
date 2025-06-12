// Answer 0

#[test]
fn test_fmt_on_lazy_with_debug() {
    use core::fmt::Formatter;

    struct MockFormatter {
        output: String,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = MockFormatter { output: String::new() };
    let once_cell = OnceCell(UnsafeCell::new(Some(42))); // using i32 as an example
    let lazy = Lazy {
        cell: once_cell,
        init: Cell::new(None),
    };
    let result = lazy.fmt(&mut formatter);
    assert!(result.is_ok());
    assert!(formatter.output.contains("Lazy"));
    assert!(formatter.output.contains("cell"));
}

#[test]
#[should_panic]
fn test_fmt_with_panic() {
    struct PanickingFormatter;

    impl fmt::Write for PanickingFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            panic!("This is a panic");
        }
    }

    let formatter = PanickingFormatter;
    let once_cell = OnceCell(UnsafeCell::new(Some(42)));
    let lazy = Lazy {
        cell: once_cell,
        init: Cell::new(None),
    };
    lazy.fmt(&formatter); // This should panic
}

