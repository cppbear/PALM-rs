pub fn get_key_value(&self) -> (&K, &V) {
        unsafe {
            let (key, value) = self.elem.as_ref();
            (key, value)
        }
    }