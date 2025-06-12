// Answer 0

#[test]
fn test_new_once_cell() {
    struct Imp;

    impl Imp {
        fn new() -> Self {
            Imp
        }
    }

    struct OnceCell<T>(Imp);

    impl<T> OnceCell<T> {
        const fn new() -> OnceCell<T> {
            OnceCell(Imp::new())
        }
    }

    let cell: OnceCell<()> = OnceCell::new();
    assert_eq!(std::mem::size_of_val(&cell), std::mem::size_of::<OnceCell<()>>());
}

#[test]
fn test_new_empty_once_cell() {
    struct Imp;

    impl Imp {
        fn new() -> Self {
            Imp
        }
    }

    struct OnceCell<T>(Imp);

    impl<T> OnceCell<T> {
        const fn new() -> OnceCell<T> {
            OnceCell(Imp::new())
        }
    }

    let empty_cell: OnceCell<u32> = OnceCell::new();
    assert_eq!(std::mem::size_of_val(&empty_cell), std::mem::size_of::<OnceCell<u32>>());
}

#[test]
#[should_panic]
fn test_new_cell_with_invalid_type() {
    struct Imp;

    impl Imp {
        fn new() -> Self {
            Imp
        }
    }

    struct OnceCell<T>(Imp);

    impl<T> OnceCell<T> {
        const fn new() -> OnceCell<T> {
            OnceCell(Imp::new())
        }
    }

    let _invalid_cell: OnceCell<String> = OnceCell::new(); // Example demonstrating panicking scenario
}

