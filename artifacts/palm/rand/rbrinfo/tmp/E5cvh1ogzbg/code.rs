fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> __m256i {
        let mut buf = [0_u8; core::mem::size_of::<__m256i>()];
        rng.fill_bytes(&mut buf);
        // x86 is little endian so no need for conversion

        // SAFETY: All byte sequences of `buf` represent values of the output type.
        unsafe { core::mem::transmute(buf) }
    }