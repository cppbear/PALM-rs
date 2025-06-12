fn is_nonfinite(self) -> bool {
        const EXP_MASK: u32 = 0x7f800000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }