fn try_get_u32(&mut self) -> Result<u32, TryGetError> {
            (**self).try_get_u32()
        }