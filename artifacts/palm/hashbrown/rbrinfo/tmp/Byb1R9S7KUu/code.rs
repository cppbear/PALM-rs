pub fn insert_entry(&mut self, hash: u64, value: T, hasher: impl Fn(&T) -> u64) -> &mut T {
        unsafe { self.insert(hash, value, hasher).as_mut() }
    }