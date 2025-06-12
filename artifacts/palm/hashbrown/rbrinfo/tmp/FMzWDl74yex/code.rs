pub fn into_key(self) -> &'a mut K {
        unsafe { &mut self.elem.as_mut().0 }
    }