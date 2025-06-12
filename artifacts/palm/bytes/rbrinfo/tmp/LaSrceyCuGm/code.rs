fn try_get_u8(&mut self) -> Result<u8, TryGetError> {
            (**self).try_get_u8()
        }