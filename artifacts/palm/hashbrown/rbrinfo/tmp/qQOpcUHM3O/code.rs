pub fn retain(&mut self, mut f: impl FnMut(&mut T) -> bool) {
        // Here we only use `iter` as a temporary, preventing use-after-free
        unsafe {
            for item in self.raw.iter() {
                if !f(item.as_mut()) {
                    self.raw.erase(item);
                }
            }
        }
    }