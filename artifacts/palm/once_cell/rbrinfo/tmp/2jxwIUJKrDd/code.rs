fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Lazy").field("cell", &self.cell).field("init", &"..").finish()
        }