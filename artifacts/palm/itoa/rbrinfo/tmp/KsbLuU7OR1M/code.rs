pub fn format<I: Integer>(&mut self, i: I) -> &str {
        let string = i.write(unsafe {
            &mut *(&mut self.bytes as *mut [MaybeUninit<u8>; i128::MAX_STR_LEN]
                as *mut <I as private::Sealed>::Buffer)
        });
        if string.len() > I::MAX_STR_LEN {
            unsafe { hint::unreachable_unchecked() };
        }
        string
    }