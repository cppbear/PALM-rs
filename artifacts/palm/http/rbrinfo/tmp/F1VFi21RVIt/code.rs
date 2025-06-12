fn is_yellow(&self) -> bool {
        matches!(*self, Danger::Yellow)
    }