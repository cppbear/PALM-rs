// Answer 0

#[test]
pub unsafe fn test_format32_zero() {
    let f: f32 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_negative_max() {
    let f: f32 = -3.4028235e+38;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_negative_large() {
    let f: f32 = -1.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_negative_small() {
    let f: f32 = -1e-40;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_negative_very_small() {
    let f: f32 = -5e-50;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

