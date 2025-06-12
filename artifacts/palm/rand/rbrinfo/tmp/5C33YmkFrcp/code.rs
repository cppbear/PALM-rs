fn fill<T: Fill + ?Sized>(&mut self, dest: &mut T) {
        dest.fill(self)
    }