fn gen_u128(&mut self) -> u128 {
        (u128::from(self.gen_u64()) << 64) | u128::from(self.gen_u64())
    }