// Answer 0

#[test]
unsafe fn test_format64_negative_zero() {
    let f = -0.0;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_negative_one() {
    let f = -1.0;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_negative_min_normal() {
    let f = -4.9406564584124654e-324;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
}

