fn try_get_u128(&mut self) -> Result<u128, TryGetError> {
            (**self).try_get_u128()
        }