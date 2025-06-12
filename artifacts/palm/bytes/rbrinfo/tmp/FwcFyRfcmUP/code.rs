fn saturating_sub_usize_u64(a: usize, b: u64) -> usize {
    use core::convert::TryFrom;
    match usize::try_from(b) {
        Ok(b) => a.saturating_sub(b),
        Err(_) => 0,
    }
}