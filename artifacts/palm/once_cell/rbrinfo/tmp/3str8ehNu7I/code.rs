pub fn get(&self) -> Option<&T> {
            if self.0.is_initialized() {
                // Safe b/c value is initialized.
                Some(unsafe { self.get_unchecked() })
            } else {
                None
            }
        }