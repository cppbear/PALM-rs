pub fn get_key_value_mut(&mut self) -> (&mut K, &mut V) {
        unsafe {
            let &mut (ref mut key, ref mut value) = self.elem.as_mut();
            (key, value)
        }
    }