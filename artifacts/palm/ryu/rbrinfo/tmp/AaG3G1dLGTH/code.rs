pub fn mul_pow5_inv_div_pow2(m: u32, q: u32, j: i32) -> u32 {
    #[cfg(feature = "small")]
    {
        // The inverse multipliers are defined as [2^x / 5^y] + 1; the upper 64
        // bits from the double lookup table are the correct bits for [2^x /
        // 5^y], so we have to add 1 here. Note that we rely on the fact that
        // the added 1 that's already stored in the table never overflows into
        // the upper 64 bits.
        let pow5 = unsafe { d2s::compute_inv_pow5(q) };
        mul_shift_32(m, pow5.1 + 1, j)
    }

    #[cfg(not(feature = "small"))]
    {
        debug_assert!(q < d2s::DOUBLE_POW5_INV_SPLIT.len() as u32);
        unsafe {
            mul_shift_32(
                m,
                d2s::DOUBLE_POW5_INV_SPLIT.get_unchecked(q as usize).1 + 1,
                j,
            )
        }
    }
}