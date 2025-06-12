fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> __m128i {
        // NOTE: It's tempting to use the u128 impl here, but confusingly this
        // results in different code (return via rdx, r10 instead of rax, rdx
        // with u128 impl) and is much slower (+130 time). This version calls
        // impls::fill_bytes_via_next but performs well.

        let mut buf = [0_u8; core::mem::size_of::<__m128i>()];
        rng.fill_bytes(&mut buf);
        // x86 is little endian so no need for conversion

        // SAFETY: All byte sequences of `buf` represent values of the output type.
        unsafe { core::mem::transmute(buf) }
    }