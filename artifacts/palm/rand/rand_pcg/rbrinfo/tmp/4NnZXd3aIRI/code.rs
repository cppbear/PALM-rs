pub fn new(state: u128, stream: u128) -> Self {
        // The increment must be odd, hence we discard one bit:
        let increment = (stream << 1) | 1;
        Self::from_state_incr(state, increment)
    }