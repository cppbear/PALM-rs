pub fn is_redirection(&self) -> bool {
        (300..400).contains(&self.0.get())
    }