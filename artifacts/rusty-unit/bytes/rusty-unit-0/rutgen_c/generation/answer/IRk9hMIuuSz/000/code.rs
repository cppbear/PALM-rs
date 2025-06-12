// Answer 0

#[test]
fn test_fmt_display() {
    struct UninitSliceWrapper(UninitSlice);

    let uninit_slice = UninitSliceWrapper(UninitSlice([MaybeUninit::uninit(); 3]));

    let mut buffer = core::fmt::Formatter::new();
    let result = uninit_slice.0.fmt(&mut buffer);
    
    assert!(result.is_ok());
}

#[test]
fn test_fmt_empty_slice() {
    struct UninitSliceWrapper(UninitSlice);

    let uninit_slice = UninitSliceWrapper(UninitSlice([]));

    let mut buffer = core::fmt::Formatter::new();
    let result = uninit_slice.0.fmt(&mut buffer);
    
    assert!(result.is_ok());
}

