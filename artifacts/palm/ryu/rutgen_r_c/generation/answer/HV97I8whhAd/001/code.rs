// Answer 0

#[test]
fn test_mul_shift_all_64() {
    // Prepare input values
    let m: u64 = 10;
    let mul: (u64, u64) = (3, 5);
    let j: u32 = 66;
    let mm_shift: u32 = 2;

    // Setup mutable storage for results
    let mut vp: u64 = 0;
    let mut vm: u64 = 0;

    // Unsafe block as the function is unsafe
    unsafe {
        let result = mul_shift_all_64(m, &mul, j, &mut vp, &mut vm, mm_shift);

        // Assert the expected outputs
        assert_eq!(vp, mul_shift_64(4 * m + 2, &mul, j));
        assert_eq!(vm, mul_shift_64(4 * m - 1 - mm_shift as u64, &mul, j));
        assert_eq!(result, mul_shift_64(4 * m, &mul, j));
    }
}

#[test]
fn test_mul_shift_all_64_boundary_cases() {
    // Testing with minimum and maximum values for u64
    let m_min: u64 = 0;
    let m_max: u64 = u64::MAX;
    let mul: (u64, u64) = (1, 1);
    let j: u32 = 64; // Edge case for shifting
    let mm_shift: u32 = 0;

    let mut vp_min: u64 = 0;
    let mut vm_min: u64 = 0;

    let mut vp_max: u64 = 0;
    let mut vm_max: u64 = 0;

    unsafe {
        let result_min = mul_shift_all_64(m_min, &mul, j, &mut vp_min, &mut vm_min, mm_shift);
        let result_max = mul_shift_all_64(m_max, &mul, j, &mut vp_max, &mut vm_max, mm_shift);

        assert_eq!(vp_min, mul_shift_64(4 * m_min + 2, &mul, j));
        assert_eq!(vm_min, mul_shift_64(4 * m_min - 1 - mm_shift as u64, &mul, j));
        assert_eq!(result_min, mul_shift_64(4 * m_min, &mul, j));

        assert_eq!(vp_max, mul_shift_64(4 * m_max + 2, &mul, j));
        assert_eq!(vm_max, mul_shift_64(4 * m_max - 1 - mm_shift as u64, &mul, j));
        assert_eq!(result_max, mul_shift_64(4 * m_max, &mul, j));
    }
}

