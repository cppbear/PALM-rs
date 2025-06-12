fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> {
            (**self).try_get_f64_le()
        }