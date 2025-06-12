fn try_get_f32(&mut self) -> Result<f32, TryGetError> {
            (**self).try_get_f32()
        }