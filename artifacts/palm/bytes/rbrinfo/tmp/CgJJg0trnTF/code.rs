fn try_get_u16(&mut self) -> Result<u16, TryGetError> {
            (**self).try_get_u16()
        }