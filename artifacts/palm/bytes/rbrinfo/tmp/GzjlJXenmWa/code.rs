fn put_f64_le(&mut self, n: f64) {
        self.put_u64_le(n.to_bits());
    }