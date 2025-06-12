const fn full(hash: u64) -> Tag {
        // Constant for function that grabs the top 7 bits of the hash.
        const MIN_HASH_LEN: usize = if mem::size_of::<usize>() < mem::size_of::<u64>() {
            mem::size_of::<usize>()
        } else {
            mem::size_of::<u64>()
        };

        // Grab the top 7 bits of the hash. While the hash is normally a full 64-bit
        // value, some hash functions (such as FxHash) produce a usize result
        // instead, which means that the top 32 bits are 0 on 32-bit platforms.
        // So we use MIN_HASH_LEN constant to handle this.
        let top7 = hash >> (MIN_HASH_LEN * 8 - 7);
        Tag((top7 & 0x7f) as u8) // truncation
    }