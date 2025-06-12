// Answer 0

#[test]
fn test_get_or_try_init_initializes() {
    use std::num::NonZeroUsize;
    use once_cell::sync::OnceCell;

    let cell: OnceCell<NonZeroUsize> = OnceCell::new();

    let result = cell.get_or_try_init(|| {
        NonZeroUsize::new(42).ok_or("Initialization failed")
    });

    assert_eq!(result, Ok(NonZeroUsize::new(42).unwrap()));
    assert_eq!(cell.get(), Some(&NonZeroUsize::new(42).unwrap()));
}

#[test]
#[should_panic(expected = "Initialization failed")]
fn test_get_or_try_init_panic() {
    use std::num::NonZeroUsize;
    use once_cell::sync::OnceCell;

    let cell: OnceCell<NonZeroUsize> = OnceCell::new();

    let _ = cell.get_or_try_init(|| {
        Err("Initialization failed")
    });

    // Since we expect this to panic on the failure of initialization,
    // we do not need further assertions.
}

#[test]
fn test_get_or_try_init_with_existing_value() {
    use std::num::NonZeroUsize;
    use once_cell::sync::OnceCell;

    let cell: OnceCell<NonZeroUsize> = OnceCell::new();
    cell.set(NonZeroUsize::new(99).unwrap()).unwrap();

    let result = cell.get_or_try_init(|| {
        NonZeroUsize::new(42).ok_or("Initialization failed")
    });

    assert_eq!(result, Ok(NonZeroUsize::new(99).unwrap()));
    assert_eq!(cell.get(), Some(&NonZeroUsize::new(99).unwrap()));
}

