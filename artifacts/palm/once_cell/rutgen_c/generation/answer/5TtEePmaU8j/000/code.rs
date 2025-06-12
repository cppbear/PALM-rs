// Answer 0

#[test]
fn test_lazy_debug_fmt() {
    use core::fmt::Formatter;

    struct TestOnceCell;

    struct TestLazy {
        cell: TestOnceCell,
        init: Cell<Option<fn() -> i32>>,
    }

    impl fmt::Debug for TestOnceCell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("OnceCell").field("inner", &"..").finish()
        }
    }

    impl TestLazy {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            f.debug_struct("Lazy").field("cell", &self.cell).field("init", &"..").finish()
        }
    }

    let lazy = TestLazy {
        cell: TestOnceCell,
        init: Cell::new(None),
    };

    let mut buffer = core::fmt::Formatter::new();
    let result = lazy.fmt(&mut buffer);

    assert!(result.is_ok());
}

