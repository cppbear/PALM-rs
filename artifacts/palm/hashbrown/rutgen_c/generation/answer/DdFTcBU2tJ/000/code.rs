// Answer 0

#[test]
fn test_alloc_err_fallible() {
    let fallibility = Fallibility::Fallible;
    let layout = Layout::from_size_align(8, 1).unwrap();
    let error = fallibility.alloc_err(layout);
    match error {
        TryReserveError::AllocError { layout: error_layout } => {
            assert_eq!(error_layout.size(), 8);
            assert_eq!(error_layout.align(), 1);
        }
        _ => panic!("Expected AllocError"),
    }
}

#[should_panic]
#[test]
fn test_alloc_err_infallible() {
    let fallibility = Fallibility::Infallible;
    let layout = Layout::from_size_align(16, 1).unwrap();
    fallibility.alloc_err(layout);
}

