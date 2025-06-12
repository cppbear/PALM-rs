// Answer 0

#[test]
fn test_mul_shift_64_case1() {
    let m = 0;
    let mul = &(1, 1);
    let j = 64;
    mul_shift_64(m, mul, j);
}

#[test]
fn test_mul_shift_64_case2() {
    let m = 1;
    let mul = &(1, 1);
    let j = 64;
    mul_shift_64(m, mul, j);
}

#[test]
fn test_mul_shift_64_case3() {
    let m = u64::MAX;
    let mul = &(1, 1);
    let j = 64;
    mul_shift_64(m, mul, j);
}

#[test]
fn test_mul_shift_64_case4() {
    let m = 0;
    let mul = &(0, 0);
    let j = 128;
    mul_shift_64(m, mul, j);
}

#[test]
fn test_mul_shift_64_case5() {
    let m = 1;
    let mul = &(0, 0);
    let j = 128;
    mul_shift_64(m, mul, j);
}

#[test]
fn test_mul_shift_64_case6() {
    let m = u64::MAX;
    let mul = &(u64::MAX, u64::MAX);
    let j = 128;
    mul_shift_64(m, mul, j);
}

#[test]
fn test_mul_shift_64_case7() {
    let m = u64::MAX;
    let mul = &(1, 1);
    let j = 128;
    mul_shift_64(m, mul, j);
}

#[test]
fn test_mul_shift_64_case8() {
    let m = 0;
    let mul = &(u64::MAX, u64::MAX);
    let j = 64;
    mul_shift_64(m, mul, j);
}

#[test]
fn test_mul_shift_64_case9() {
    let m = 1;
    let mul = &(u64::MAX, u64::MAX);
    let j = 64;
    mul_shift_64(m, mul, j);
}

