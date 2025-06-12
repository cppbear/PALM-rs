// Answer 0

#[test]
fn test_alloc_err_fallible() {
    use std::alloc::Layout;
    use std::alloc::handle_alloc_error;

    struct Fallibility;
    
    impl Fallibility {
        const FALLIBLE: Self = Fallibility;

        fn alloc_err(self, layout: Layout) -> TryReserveError {
            match self {
                _ => TryReserveError::AllocError { layout },
            }
        }
    }

    let layout = Layout::from_size_align(1, 1).unwrap();
    let result = Fallibility::FALLIBLE.alloc_err(layout);
    
    match result {
        TryReserveError::AllocError { .. } => (),
        _ => panic!("Expected AllocError"),
    }
}

#[test]
#[should_panic]
fn test_alloc_err_infallible() {
    use std::alloc::Layout;
    use std::alloc::handle_alloc_error;

    struct Fallibility;
    
    impl Fallibility {
        const INFALLIBLE: Self = Fallibility;

        fn alloc_err(self, layout: Layout) -> TryReserveError {
            match self {
                _ => handle_alloc_error(layout),
            }
        }
    }

    let layout = Layout::from_size_align(1, 1).unwrap();
    Fallibility::INFALLIBLE.alloc_err(layout);
}

