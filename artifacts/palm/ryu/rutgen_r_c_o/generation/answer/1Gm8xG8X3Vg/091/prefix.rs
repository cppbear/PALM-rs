// Answer 0

#[test]
fn test_format64_zero_positive() {
    let f: f64 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format64_zero_negative() {
    let f: f64 = -0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format64_negative_one() {
    let f: f64 = -1.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format64_one() {
    let f: f64 = 1.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format64_max_boundary() {
    let f: f64 = 0.0000000000000001;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format64_min_boundary() {
    let f: f64 = -0.0000000000000001;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr() as *mut u8);
    }
}

