fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> {
            (**self).try_get_f32_le()
        }