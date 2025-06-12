fn is_red(&self) -> bool {
        matches!(*self, Danger::Red(_))
    }