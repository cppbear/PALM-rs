fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> {
            (**self).try_get_i16_le()
        }