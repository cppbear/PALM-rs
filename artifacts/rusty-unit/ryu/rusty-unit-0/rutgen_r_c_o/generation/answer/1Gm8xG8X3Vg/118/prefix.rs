// Answer 0

#[test]
fn test_format64_negative_max() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = format64(-8.98846567431158e307, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format64_negative_mid() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = format64(-1.0, buffer.as_mut_ptr() as *mut u8);
    }
}

#[test]
fn test_format64_negative_small() {
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    unsafe {
        let len = format64(-1e-10, buffer.as_mut_ptr() as *mut u8);
    }
}

