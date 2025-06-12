// Answer 0

#[test]
pub unsafe fn test_format32_positive_normalized() {
    let f: f32 = 1.0;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_positive_small() {
    let f: f32 = 1.5;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_positive_large() {
    let f: f32 = 3.4028235e38;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_positive_mid() {
    let f: f32 = 2.5;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

