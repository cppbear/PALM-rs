pub fn is_informational(&self) -> bool {
        (100..200).contains(&self.0.get())
    }