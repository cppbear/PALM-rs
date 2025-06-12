// Answer 0

#[test]
fn test_force_evaluates_lazy_value() {
    struct SimpleFn;

    impl FnOnce<()> for SimpleFn {
        type Output = i32;

        extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
            42
        }
    }

    let lazy = Lazy {
        cell: OnceCell(UnsafeCell::new(None)),
        init: Cell::new(Some(SimpleFn)),
    };

    assert_eq!(Lazy::force(&lazy), &42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_panics_when_init_is_none() {
    let lazy = Lazy {
        cell: OnceCell(UnsafeCell::new(None)),
        init: Cell::new(None),
    };

    Lazy::force(&lazy);
}

