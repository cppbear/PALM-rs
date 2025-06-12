pub fn into_inner(self) -> Option<T> {
            // Because `into_inner` takes `self` by value, the compiler statically verifies
            // that it is not currently borrowed. So it is safe to move out `Option<T>`.
            self.inner.into_inner()
        }