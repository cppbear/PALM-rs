fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
                // Floats have an explicit representation for overflow
                *self += *v;
                Ok(())
            }