use core::ptr;
#[cfg_attr(feature = "no-panic", inline)]
pub(crate) fn pow5_factor(mut value: u64) -> u32 {
    const M_INV_5: u64 = 14757395258967641293;
    const N_DIV_5: u64 = 3689348814741910323;
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
