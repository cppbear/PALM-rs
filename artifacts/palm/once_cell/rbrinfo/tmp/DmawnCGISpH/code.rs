fn deref_mut(&mut self) -> &mut T {
            Lazy::force_mut(self)
        }