pub fn sample_weighted<R, F, X>(
    rng: &mut R,
    length: usize,
    weight: F,
    amount: usize,
) -> Result<IndexVec, WeightError>
where
    R: Rng + ?Sized,
    F: Fn(usize) -> X,
    X: Into<f64>,
{
    if length > (u32::MAX as usize) {
        #[cfg(target_pointer_width = "32")]
        unreachable!();

        #[cfg(target_pointer_width = "64")]
        {
            let amount = amount as u64;
            let length = length as u64;
            sample_efraimidis_spirakis(rng, length, weight, amount)
        }
    } else {
        assert!(amount <= u32::MAX as usize);
        let amount = amount as u32;
        let length = length as u32;
        sample_efraimidis_spirakis(rng, length, weight, amount)
    }
}