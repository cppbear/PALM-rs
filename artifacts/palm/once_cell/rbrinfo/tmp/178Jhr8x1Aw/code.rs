pub fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {
            let mut value = Some(value);
            let res = self.get_or_init(|| unsafe { value.take().unwrap_unchecked() });
            match value {
                None => Ok(res),
                Some(value) => Err((res, value)),
            }
        }