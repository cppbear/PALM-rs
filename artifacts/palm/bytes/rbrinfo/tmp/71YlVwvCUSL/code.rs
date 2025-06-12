fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
            (**self).try_get_uint_le(nbytes)
        }