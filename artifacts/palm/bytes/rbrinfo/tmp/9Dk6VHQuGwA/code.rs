fn try_get_int_ne(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
            (**self).try_get_int_ne(nbytes)
        }