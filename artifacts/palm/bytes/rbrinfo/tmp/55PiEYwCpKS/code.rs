fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError> {
            (**self).try_get_u32_ne()
        }