fn start_ptr(&self, si: StatePtr) -> StatePtr {
        if self.has_prefix() {
            si | STATE_START
        } else {
            si
        }
    }