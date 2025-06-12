fn put_f32_ne(&mut self, n: f32) {
        self.put_u32_ne(n.to_bits());
    }