// Answer 0

#[test]
fn test_mul_shift_all_64() {
    use std::ptr;

    struct MulShiftTest;

    unsafe fn mul_shift_64(m: u64, mul: &(u64, u64), j: u32) -> u64 {
        (m * mul.0 + (m >> j) * mul.1) >> j
    }

    let m: u64 = 5;
    let mul: (u64, u64) = (2, 3);
    let j: u32 = 1;

    let mut vp: u64 = 0;
    let mut vm: u64 = 0;
    let mm_shift: u32 = 2;

    let result = mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);

    assert_eq!(result, mul_shift_64(4 * m, &mul, j));
    assert_eq!(vp, mul_shift_64(4 * m + 2, &mul, j));
    assert_eq!(vm, mul_shift_64(4 * m - 1 - mm_shift as u64, &mul, j));
}

#[test]
fn test_mul_shift_all_64_boundary() {
    use std::ptr;

    struct MulShiftTest;

    unsafe fn mul_shift_64(m: u64, mul: &(u64, u64), j: u32) -> u64 {
        (m * mul.0 + (m >> j) * mul.1) >> j
    }

    let m: u64 = 0;
    let mul: (u64, u64) = (0, 0);
    let j: u32 = 0;

    let mut vp: u64 = 0;
    let mut vm: u64 = 0;
    let mm_shift: u32 = 0;

    let result = mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);

    assert_eq!(result, mul_shift_64(4 * m, &mul, j));
    assert_eq!(vp, mul_shift_64(4 * m + 2, &mul, j));
    assert_eq!(vm, mul_shift_64(4 * m - 1 - mm_shift as u64, &mul, j));
}

