// Answer 0

#[test]
fn test_lazy_fmt_debug() {
    struct Dummy<T>(Option<T>);

    impl<T: fmt::Debug> fmt::Debug for Dummy<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    let lazy = Lazy {
        cell: OnceCell(Dummy(Some("test"))),
        init: Cell::new(None),
    };

    let result = format!("{:?}", lazy);
    assert_eq!(result, "Lazy { cell: Some(\"test\"), init: .. }");
}

#[test]
fn test_lazy_fmt_debug_with_none() {
    struct Dummy<T>(Option<T>);

    impl<T: fmt::Debug> fmt::Debug for Dummy<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    let lazy = Lazy {
        cell: OnceCell(Dummy(None)),
        init: Cell::new(None),
    };

    let result = format!("{:?}", lazy);
    assert_eq!(result, "Lazy { cell: None, init: .. }");
}

