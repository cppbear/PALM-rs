fn set_green(&mut self) {
        debug_assert!(self.is_yellow());
        *self = Danger::Green;
    }