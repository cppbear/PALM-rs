fn set_red(&mut self) {
        debug_assert!(self.is_yellow());
        *self = Danger::Red(RandomState::new());
    }