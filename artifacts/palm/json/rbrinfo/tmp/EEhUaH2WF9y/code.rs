pub fn is_data(&self) -> bool {
        self.classify() == Category::Data
    }