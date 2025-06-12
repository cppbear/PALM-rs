fn try_get_u64(&mut self) -> Result<u64, TryGetError> {
            (**self).try_get_u64()
        }