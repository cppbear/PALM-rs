fn put_f64_ne(&mut self, n: f64) {
        self.put_u64_ne(n.to_bits());
    }