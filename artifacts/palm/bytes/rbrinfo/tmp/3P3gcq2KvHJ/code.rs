fn try_get_i64(&mut self) -> Result<i64, TryGetError> {
            (**self).try_get_i64()
        }