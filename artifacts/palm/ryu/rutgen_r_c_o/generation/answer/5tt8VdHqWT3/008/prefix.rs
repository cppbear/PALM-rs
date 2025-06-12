// Answer 0

#[test]
fn test_write_mantissa_case_0() {
    let mut buffer = [0u8; 10];
    unsafe {
        write_mantissa(0, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_case_1() {
    let mut buffer = [0u8; 10];
    unsafe {
        write_mantissa(1, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_case_2() {
    let mut buffer = [0u8; 10];
    unsafe {
        write_mantissa(2, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_case_3() {
    let mut buffer = [0u8; 10];
    unsafe {
        write_mantissa(3, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_case_4() {
    let mut buffer = [0u8; 10];
    unsafe {
        write_mantissa(4, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_case_5() {
    let mut buffer = [0u8; 10];
    unsafe {
        write_mantissa(5, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_case_6() {
    let mut buffer = [0u8; 10];
    unsafe {
        write_mantissa(6, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_case_7() {
    let mut buffer = [0u8; 10];
    unsafe {
        write_mantissa(7, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_case_8() {
    let mut buffer = [0u8; 10];
    unsafe {
        write_mantissa(8, buffer.as_mut_ptr().add(10));
    }
}

#[test]
fn test_write_mantissa_case_9() {
    let mut buffer = [0u8; 10];
    unsafe {
        write_mantissa(9, buffer.as_mut_ptr().add(10));
    }
}

