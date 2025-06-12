pub fn is_eof(&self) -> bool {
        self.classify() == Category::Eof
    }