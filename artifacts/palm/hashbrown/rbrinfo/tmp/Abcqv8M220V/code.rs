pub fn key(&self) -> &K {
        unsafe { &self.elem.as_ref().0 }
    }