fn put_f32_le(&mut self, n: f32) {
        self.put_u32_le(n.to_bits());
    }