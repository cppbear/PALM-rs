fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
                match self.checked_add(*v) {
                    Some(sum) => {
                        *self = sum;
                        Ok(())
                    }
                    None => Err(()),
                }
            }