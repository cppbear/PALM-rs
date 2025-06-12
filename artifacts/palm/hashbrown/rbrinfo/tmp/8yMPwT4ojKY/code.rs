pub fn into_mut(self) -> &'a mut V {
        unsafe { &mut self.elem.as_mut().1 }
    }