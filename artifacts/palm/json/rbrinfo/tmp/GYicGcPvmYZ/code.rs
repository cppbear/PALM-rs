pub fn is_io(&self) -> bool {
        self.classify() == Category::Io
    }