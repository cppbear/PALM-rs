pub fn get(&self) -> &V {
        unsafe { &self.elem.as_ref().1 }
    }