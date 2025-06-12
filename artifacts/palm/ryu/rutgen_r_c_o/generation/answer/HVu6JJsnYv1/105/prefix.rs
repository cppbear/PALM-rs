// Answer 0

#[test]
fn test_format32_zero() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(0.0f32, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_negative_zero() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(-0.0f32, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_small_positive() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(1e-45f32, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_small_negative() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(-1e-45f32, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_smallest_positive() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(1.0e-45f32, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_smallest_negative() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(-1.0e-45f32, buffer.as_mut_ptr() as *mut u8);
    }
}

