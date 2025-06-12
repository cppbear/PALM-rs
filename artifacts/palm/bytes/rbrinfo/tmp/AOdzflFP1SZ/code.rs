fn try_get_u128_le(&mut self) -> Result<u128, TryGetError> {
            (**self).try_get_u128_le()
        }