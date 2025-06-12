fn default() -> Self {
        // SAFETY: Because the table is static, it always outlives the iter.
        unsafe { RawTableInner::NEW.iter() }
    }