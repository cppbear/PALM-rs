fn try_get_i128(&mut self) -> Result<i128, TryGetError> {
            (**self).try_get_i128()
        }