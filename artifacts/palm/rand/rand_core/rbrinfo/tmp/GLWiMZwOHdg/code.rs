fn seed_from_u64(mut state: u64) -> Self {
        // We use PCG32 to generate a u32 sequence, and copy to the seed
        fn pcg32(state: &mut u64) -> [u8; 4] {
            const MUL: u64 = 6364136223846793005;
            const INC: u64 = 11634580027462260723;

            // We advance the state first (to get away from the input value,
            // in case it has low Hamming Weight).
            *state = state.wrapping_mul(MUL).wrapping_add(INC);
            let state = *state;

            // Use PCG output function with to_le to generate x:
            let xorshifted = (((state >> 18) ^ state) >> 27) as u32;
            let rot = (state >> 59) as u32;
            let x = xorshifted.rotate_right(rot);
            x.to_le_bytes()
        }

        let mut seed = Self::Seed::default();
        let mut iter = seed.as_mut().chunks_exact_mut(4);
        for chunk in &mut iter {
            chunk.copy_from_slice(&pcg32(&mut state));
        }
        let rem = iter.into_remainder();
        if !rem.is_empty() {
            rem.copy_from_slice(&pcg32(&mut state)[..rem.len()]);
        }

        Self::from_seed(seed)
    }