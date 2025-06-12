fn try_get_i16(&mut self) -> Result<i16, TryGetError> {
            (**self).try_get_i16()
        }