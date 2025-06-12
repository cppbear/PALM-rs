pub fn set(&self, value: NonZeroUsize) -> Result<(), ()> {
        match self.compare_exchange(value) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }