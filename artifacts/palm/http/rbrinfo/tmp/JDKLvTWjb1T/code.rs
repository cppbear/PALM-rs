pub fn as_str(&self) -> &str {
            // Safety: the invariant of AllocatedExtension ensures that self.0
            // contains valid UTF-8.
            unsafe { str::from_utf8_unchecked(&self.0) }
        }