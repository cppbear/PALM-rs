fn next_u64(&mut self) -> u64 {
        self.step();
        output_xsl_rr(self.state)
    }