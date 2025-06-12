fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError> {
            (**self).try_get_u64_ne()
        }