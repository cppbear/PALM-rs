pub fn into_key_value(self) -> (&'a mut K, &'a mut V) {
        unsafe {
            let &mut (ref mut key, ref mut value) = self.elem.as_mut();
            (key, value)
        }
    }