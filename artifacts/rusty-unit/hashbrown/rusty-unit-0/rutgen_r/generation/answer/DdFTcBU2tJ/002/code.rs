// Answer 0

#[derive(Debug)]
enum Fallibility {
    Fallible,
    Infallible,
}

#[derive(Debug)]
struct Layout {
    size: usize,
    align: usize,
}

#[derive(Debug)]
struct TryReserveError {
    alloc_error: Option<Layout>,
}

impl TryReserveError {
    fn AllocError(layout: Layout) -> Self {
        TryReserveError {
            alloc_error: Some(layout),
        }
    }
}

impl Fallibility {
    fn alloc_err(self, layout: Layout) -> TryReserveError {
        match self {
            Fallibility::Fallible => TryReserveError::AllocError(layout),
            Fallibility::Infallible => panic!("Infallible allocator should not fail"),
        }
    }
}

#[test]
fn test_alloc_err_fallible_case() {
    let fallible = Fallibility::Fallible;
    let layout = Layout { size: 32, align: 8 }; // boundary case for typical allocation
    let result = fallible.alloc_err(layout);
    match result.alloc_error {
        Some(ref l) if l.size == 32 && l.align == 8 => assert!(true),
        _ => assert!(false, "Expected AllocError with specified layout"),
    }
}

#[test]
fn test_alloc_err_large_size() {
    let fallible = Fallibility::Fallible;
    let layout = Layout { size: usize::MAX, align: 8 }; // edge case for maximum size
    let result = fallible.alloc_err(layout);
    match result.alloc_error {
        Some(ref l) if l.size == usize::MAX && l.align == 8 => assert!(true),
        _ => assert!(false, "Expected AllocError with specified layout"),
    }
}

#[test]
fn test_alloc_err_zero_size() {
    let fallible = Fallibility::Fallible;
    let layout = Layout { size: 0, align: 8 }; // boundary case with zero size
    let result = fallible.alloc_err(layout);
    match result.alloc_error {
        Some(ref l) if l.size == 0 && l.align == 8 => assert!(true),
        _ => assert!(false, "Expected AllocError with specified layout"),
    }
}

