fn continue_past_first_match(&self) -> bool {
        self.prog.is_reverse || self.prog.matches.len() > 1
    }