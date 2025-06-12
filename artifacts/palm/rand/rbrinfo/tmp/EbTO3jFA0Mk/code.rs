pub fn sample_array<R, const N: usize>(rng: &mut R, len: usize) -> Option<[usize; N]>
    where
        R: Rng + ?Sized,
    {
        if N > len {
            return None;
        }

        // Floyd's algorithm
        let mut indices = [0; N];
        for (i, j) in (len - N..len).enumerate() {
            let t = rng.random_range(..j + 1);
            if let Some(pos) = indices[0..i].iter().position(|&x| x == t) {
                indices[pos] = j;
            }
            indices[i] = t;
        }
        Some(indices)
    }