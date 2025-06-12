fn get_int(&mut self, nbytes: usize) -> i64 {
        sign_extend(self.get_uint(nbytes), nbytes)
    }