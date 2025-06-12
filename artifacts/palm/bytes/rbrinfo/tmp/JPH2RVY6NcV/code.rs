fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> {
            (**self).try_get_u16_le()
        }