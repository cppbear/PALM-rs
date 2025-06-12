pub fn key_mut(&mut self) -> &mut K {
        unsafe { &mut self.elem.as_mut().0 }
    }