fn output_dxsm(state: u128) -> u64 {
    // See https://github.com/imneme/pcg-cpp/blob/ffd522e7188bef30a00c74dc7eb9de5faff90092/include/pcg_random.hpp#L1016
    // for a short discussion of the construction and its original implementation.
    let mut hi = (state >> 64) as u64;
    let mut lo = state as u64;

    lo |= 1;
    hi ^= hi >> 32;
    hi = hi.wrapping_mul(MULTIPLIER);
    hi ^= hi >> 48;
    hi = hi.wrapping_mul(lo);

    hi
}