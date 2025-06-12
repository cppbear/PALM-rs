fn try_get_i32(&mut self) -> Result<i32, TryGetError> {
            (**self).try_get_i32()
        }