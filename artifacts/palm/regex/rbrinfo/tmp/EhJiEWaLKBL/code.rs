fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StateFlags")
         .field("is_match", &self.is_match())
         .field("is_word", &self.is_word())
         .field("has_empty", &self.has_empty())
         .finish()
    }