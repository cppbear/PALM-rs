fn try_get_i8(&mut self) -> Result<i8, TryGetError> {
            (**self).try_get_i8()
        }