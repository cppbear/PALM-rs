fn put_f64(&mut self, n: f64) {
        self.put_u64(n.to_bits());
    }