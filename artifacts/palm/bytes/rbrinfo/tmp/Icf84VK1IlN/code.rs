fn try_get_i128_le(&mut self) -> Result<i128, TryGetError> {
            (**self).try_get_i128_le()
        }