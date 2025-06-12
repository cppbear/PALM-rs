// Answer 0

#[test]
fn test_alloc_err_infallible_case() {
    use core::alloc::Layout;

    let fallibility = Fallibility::Infallible;
    let layout = Layout::from_size_align(1, 1).unwrap(); // size > 0, align > 0

    let _ = fallibility.alloc_err(layout);
}

#[should_panic]
fn test_alloc_err_infallible_case_layout_fail() {
    use core::alloc::Layout;

    let fallibility = Fallibility::Infallible;
    let layout = Layout::from_size_align(0, 1).unwrap(); // size = 0, should panic

    let _ = fallibility.alloc_err(layout);
}

#[test]
fn test_alloc_err_infallible_case_large_layout() {
    use core::alloc::Layout;

    let fallibility = Fallibility::Infallible;
    let layout = Layout::from_size_align(1024, 8).unwrap(); // size = 1024, align = 8

    let _ = fallibility.alloc_err(layout);
}

