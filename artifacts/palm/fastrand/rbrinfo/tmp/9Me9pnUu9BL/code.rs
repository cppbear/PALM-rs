fn gen_mod_u128(&mut self, n: u128) -> u128 {
        // Adapted from: https://lemire.me/blog/2016/06/30/fast-random-shuffling/
        let mut r = self.gen_u128();
        let mut hi = mul_high_u128(r, n);
        let mut lo = r.wrapping_mul(n);
        if lo < n {
            let t = n.wrapping_neg() % n;
            while lo < t {
                r = self.gen_u128();
                hi = mul_high_u128(r, n);
                lo = r.wrapping_mul(n);
            }
        }
        hi
    }