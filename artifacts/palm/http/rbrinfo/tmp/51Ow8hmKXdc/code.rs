pub unsafe fn from_utf8_unchecked(bytes: Bytes) -> ByteStr {
        if cfg!(debug_assertions) {
            match str::from_utf8(&bytes) {
                Ok(_) => (),
                Err(err) => panic!(
                    "ByteStr::from_utf8_unchecked() with invalid bytes; error = {}, bytes = {:?}",
                    err, bytes
                ),
            }
        }
        // Invariant: assumed by the safety requirements of this function.
        ByteStr { bytes }
    }