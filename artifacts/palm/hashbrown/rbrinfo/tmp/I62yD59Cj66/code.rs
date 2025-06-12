pub fn get_mut(&mut self) -> &mut V {
        unsafe { &mut self.elem.as_mut().1 }
    }