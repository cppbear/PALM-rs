fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Literals")
         .field("lits", &self.lits)
         .field("limit_size", &self.limit_size)
         .field("limit_class", &self.limit_class)
         .finish()
    }