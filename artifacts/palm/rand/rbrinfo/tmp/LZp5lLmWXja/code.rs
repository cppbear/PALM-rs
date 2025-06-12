pub fn sample<R>(rng: &mut R, length: usize, amount: usize) -> IndexVec
where
    R: Rng + ?Sized,
{
    if amount > length {
        panic!("`amount` of samples must be less than or equal to `length`");
    }
    if length > (u32::MAX as usize) {
        #[cfg(target_pointer_width = "32")]
        unreachable!();

        // We never want to use inplace here, but could use floyd's alg
        // Lazy version: always use the cache alg.
        #[cfg(target_pointer_width = "64")]
        return sample_rejection(rng, length as u64, amount as u64);
    }
    let amount = amount as u32;
    let length = length as u32;

    // Choice of algorithm here depends on both length and amount. See:
    // https://github.com/rust-random/rand/pull/479
    // We do some calculations with f32. Accuracy is not very important.

    if amount < 163 {
        const C: [[f32; 2]; 2] = [[1.6, 8.0 / 45.0], [10.0, 70.0 / 9.0]];
        let j = usize::from(length >= 500_000);
        let amount_fp = amount as f32;
        let m4 = C[0][j] * amount_fp;
        // Short-cut: when amount < 12, floyd's is always faster
        if amount > 11 && (length as f32) < (C[1][j] + m4) * amount_fp {
            sample_inplace(rng, length, amount)
        } else {
            sample_floyd(rng, length, amount)
        }
    } else {
        const C: [f32; 2] = [270.0, 330.0 / 9.0];
        let j = usize::from(length >= 500_000);
        if (length as f32) < C[j] * (amount as f32) {
            sample_inplace(rng, length, amount)
        } else {
            sample_rejection(rng, length, amount)
        }
    }
}