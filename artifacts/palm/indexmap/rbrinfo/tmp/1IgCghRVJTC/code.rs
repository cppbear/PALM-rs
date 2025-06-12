fn eq(&self, other: &[U; N]) -> bool {
        <Self as PartialEq<[U]>>::eq(self, other)
    }