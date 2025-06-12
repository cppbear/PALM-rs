fn write_checked(src: &[u8], dst: &mut [u8]) -> Result<(), InvalidMethod> {
        for (i, &b) in src.iter().enumerate() {
            let b = METHOD_CHARS[b as usize];

            if b == 0 {
                return Err(InvalidMethod::new());
            }

            dst[i] = b;
        }

        Ok(())
    }