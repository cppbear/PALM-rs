fn min_u64_usize(a: u64, b: usize) -> usize {
    use core::convert::TryFrom;
    match usize::try_from(a) {
        Ok(a) => usize::min(a, b),
        Err(_) => b,
    }
}