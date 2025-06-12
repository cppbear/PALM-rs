fn deref(&self) -> &T {
            Lazy::force(self)
        }