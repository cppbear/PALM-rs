fn try_get_uint(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
            (**self).try_get_uint(nbytes)
        }