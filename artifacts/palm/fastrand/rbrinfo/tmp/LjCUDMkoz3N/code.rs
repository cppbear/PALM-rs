fn gen_u64(&mut self) -> u64 {
        // Constants for WyRand taken from: https://github.com/wangyi-fudan/wyhash/blob/master/wyhash.h#L151
        // Updated for the final v4.2 implementation with improved constants for better entropy output.
        const WY_CONST_0: u64 = 0x2d35_8dcc_aa6c_78a5;
        const WY_CONST_1: u64 = 0x8bb8_4b93_962e_acc9;

        let s = self.0.wrapping_add(WY_CONST_0);
        self.0 = s;
        let t = u128::from(s) * u128::from(s ^ WY_CONST_1);
        (t as u64) ^ (t >> 64) as u64
    }