pub(crate) fn pow5_factor(mut value: u64) -> u32 {
    const M_INV_5: u64 = 14757395258967641293; // 5 * m_inv_5 = 1 (mod 2^64)
    const N_DIV_5: u64 = 3689348814741910323; // #{ n | n = 0 (mod 2^64) } = 2^64 / 5
    let mut count = 0u32;
    loop {
        debug_assert!(value != 0);
        value = value.wrapping_mul(M_INV_5);
        if value > N_DIV_5 {
            break;
        }
        count += 1;
    }
    count
}