pub fn is_syntax(&self) -> bool {
        self.classify() == Category::Syntax
    }