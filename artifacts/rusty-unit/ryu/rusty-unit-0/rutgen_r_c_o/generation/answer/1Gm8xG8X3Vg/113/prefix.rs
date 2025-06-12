// Answer 0

#[test]
fn test_format64_negative_small_float_case_1() {
    let f = -2.2250738585072014e-308;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format64_negative_small_float_case_2() {
    let f = -2.2250738585072013e-308;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format64_negative_small_float_case_3() {
    let f = -2.2250738585072012e-308;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format64_negative_small_float_case_4() {
    let f = -2.2250738585072011e-308;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format64_negative_small_float_case_5() {
    let f = -2.2250738585072010e-308;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
    }
}

