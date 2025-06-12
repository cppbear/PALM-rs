fn compiler(&self) -> Compiler {
        Compiler::new().size_limit(self.flag_size_limit)
    }