fn eq(&self, other: &HdrName<'a>) -> bool {
        match self.inner {
            Repr::Standard(a) => match other.inner {
                Repr::Standard(b) => a == b,
                _ => false,
            },
            Repr::Custom(Custom(ref a)) => match other.inner {
                Repr::Custom(ref b) => {
                    if b.lower {
                        a.as_bytes() == b.buf
                    } else {
                        eq_ignore_ascii_case(a.as_bytes(), b.buf)
                    }
                }
                _ => false,
            },
        }
    }