fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_cut() {
            write!(f, "Cut({})", escape_unicode(&self.v))
        } else {
            write!(f, "Complete({})", escape_unicode(&self.v))
        }
    }