// Answer 0

#[test]
fn test_format32_zero() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(0.0f32, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_small_positive() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(0.1f32, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_small_positive_boundary() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(0.000001f32, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_large_positive() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(0.9999999f32, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_near_zero() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(0.0000001f32, buffer.as_mut_ptr() as *mut u8);
    }
}

