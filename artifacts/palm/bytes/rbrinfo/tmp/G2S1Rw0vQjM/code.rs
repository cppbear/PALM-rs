fn put_f32(&mut self, n: f32) {
        self.put_u32(n.to_bits());
    }