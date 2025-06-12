fn decrement_depth(&mut self) {
        // Assuming the correctness of the visitor, this should never drop
        // below 0.
        self.depth = self.depth.checked_sub(1).unwrap();
    }