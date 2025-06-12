fn try_insert_entry(
        &mut self,
        hash: HashValue,
        key: HeaderName,
        value: T,
    ) -> Result<(), MaxSizeReached> {
        if self.entries.len() >= MAX_SIZE {
            return Err(MaxSizeReached::new());
        }

        self.entries.push(Bucket {
            hash,
            key,
            value,
            links: None,
        });

        Ok(())
    }