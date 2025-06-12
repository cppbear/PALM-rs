fn get_int_le(&mut self, nbytes: usize) -> i64 {
        sign_extend(self.get_uint_le(nbytes), nbytes)
    }