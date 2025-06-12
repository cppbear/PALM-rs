fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> {
            (**self).try_get_u64_le()
        }