fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError> {
            (**self).try_get_i32_ne()
        }