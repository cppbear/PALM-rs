fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(MULTIPLIER);
        output_xsl_rr(self.state)
    }