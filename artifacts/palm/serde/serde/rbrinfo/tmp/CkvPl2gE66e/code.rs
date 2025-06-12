pub fn end(self) -> Result<(), E> {
        let remaining = self.iter.count();
        if remaining == 0 {
            Ok(())
        } else {
            // First argument is the number of elements in the data, second
            // argument is the number of elements expected by the Deserialize.
            Err(de::Error::invalid_length(
                self.count + remaining,
                &ExpectedInMap(self.count),
            ))
        }
    }