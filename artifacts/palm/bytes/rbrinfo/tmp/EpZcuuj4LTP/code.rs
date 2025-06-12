fn try_get_int_le(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
            (**self).try_get_int_le(nbytes)
        }