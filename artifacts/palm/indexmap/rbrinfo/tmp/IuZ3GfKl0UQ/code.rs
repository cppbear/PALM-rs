fn eq(&self, other: &Slice<K2, V2>) -> bool {
        <[_] as PartialEq<_>>::eq(self, other)
    }