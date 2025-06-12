// Answer 0

#[test]
fn test_format32_negative_zero_mantissa() {
    let f: f32 = -0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_negative_small_mantissa() {
    let f: f32 = -0.0001;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_negative_one() {
    let f: f32 = -1.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_negative_two() {
    let f: f32 = -2.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format32_negative_point_zero_one() {
    let f: f32 = -0.01;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
    }
}

