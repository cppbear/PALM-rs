pub fn set(&self, value: &'a T) -> Result<(), ()> {
        match self.compare_exchange(value) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }