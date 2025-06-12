fn try_get_f64(&mut self) -> Result<f64, TryGetError> {
            (**self).try_get_f64()
        }