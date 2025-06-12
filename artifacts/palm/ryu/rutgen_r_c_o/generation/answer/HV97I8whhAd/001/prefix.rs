// Answer 0

#[test]
fn test_mul_shift_all_64_case1() {
    let mut vp = 0u64;
    let mut vm = 0u64;
    let m = 1u64;
    let mul = (1u64, 1u64);
    let j = 64;
    let mm_shift = 0;
    unsafe {
        mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);
    }
}

#[test]
fn test_mul_shift_all_64_case2() {
    let mut vp = 0u64;
    let mut vm = 0u64;
    let m = 18446744073709551615u64;
    let mul = (18446744073709551615u64, 18446744073709551615u64);
    let j = 128;
    let mm_shift = 63;
    unsafe {
        mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);
    }
}

#[test]
fn test_mul_shift_all_64_case3() {
    let mut vp = 0u64;
    let mut vm = 0u64;
    let m = 0u64;
    let mul = (0u64, 0u64);
    let j = 65;
    let mm_shift = 0;
    unsafe {
        mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);
    }
}

#[test]
fn test_mul_shift_all_64_case4() {
    let mut vp = 0u64;
    let mut vm = 0u64;
    let m = 100u64;
    let mul = (200u64, 300u64);
    let j = 100;
    let mm_shift = 15;
    unsafe {
        mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);
    }
}

#[test]
fn test_mul_shift_all_64_case5() {
    let mut vp = 0u64;
    let mut vm = 0u64;
    let m = 10u64;
    let mul = (500u64, 600u64);
    let j = 128;
    let mm_shift = 59;
    unsafe {
        mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);
    }
}

