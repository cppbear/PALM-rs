fn eq(&self, other: &[(K2, V2); N]) -> bool {
        <Self as PartialEq<[_]>>::eq(self, other)
    }