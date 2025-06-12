fn try_get_i64_le(&mut self) -> Result<i64, TryGetError> {
            (**self).try_get_i64_le()
        }