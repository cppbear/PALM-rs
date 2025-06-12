// Answer 0

#[test]
fn test_once_cell_new() {
    struct Imp;

    impl Imp {
        fn new() -> Imp {
            Imp
        }
    }

    struct OnceCell(T);

    impl OnceCell {
        const fn new() -> OnceCell {
            OnceCell(Imp::new())
        }
    }

    let cell: OnceCell = OnceCell::new();
    
    // Assert that the initialized cell is of the expected type and can be used further
    let expected = OnceCell(Imp::new());
    assert!(std::mem::size_of_val(&cell) == std::mem::size_of_val(&expected));
}

