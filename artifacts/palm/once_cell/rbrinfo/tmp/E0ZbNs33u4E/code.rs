pub fn take(&mut self) -> Option<T> {
            mem::take(self).into_inner()
        }