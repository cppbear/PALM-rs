// Answer 0

#[test]
#[should_panic]
fn test_mul_pow5_div_pow2_panic_invalid_i() {
    struct DummyD2S;

    impl DummyD2S {
        const DOUBLE_POW5_SPLIT: [(u32, u32); 10] = [
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8),
            (9, 9),
            (10, 10),
        ];
    }

    unsafe fn mul_shift_32(m: u32, pow5: u32, j: i32) -> u32 {
        // Simple implementation for testing
        m * pow5 >> j
    }

    let m = 10;
    let j = 2;
    let i = DummyD2S::DOUBLE_POW5_SPLIT.len() as u32; // boundary condition causing panic
    let _ = mul_pow5_div_pow2(m, i, j);
}

#[test]
fn test_mul_pow5_div_pow2_valid_case_small_feature() {
    let m = 10;
    let j = 2;
    let i = 5; // valid i in range

    #[cfg(feature = "small")]
    {
        let pow5 = unsafe { DummyD2S::DOUBLE_POW5_SPLIT[i as usize] }; // using provided data
        let result = mul_shift_32(m, pow5.1, j);
        assert_eq!(result, (m * pow5.1) >> j);
    }
}

#[test]
fn test_mul_pow5_div_pow2_valid_case_not_small_feature() {
    let m = 10;
    let j = 2;
    let i = 5; // valid i in range

    #[cfg(not(feature = "small"))]
    {
        let result = unsafe { mul_shift_32(m, DummyD2S::DOUBLE_POW5_SPLIT[i as usize].1, j) };
        assert_eq!(result, (m * DummyD2S::DOUBLE_POW5_SPLIT[i as usize].1) >> j);
    }
}

