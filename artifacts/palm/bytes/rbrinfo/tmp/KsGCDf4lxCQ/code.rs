fn try_get_u32_le(&mut self) -> Result<u32, TryGetError> {
            (**self).try_get_u32_le()
        }