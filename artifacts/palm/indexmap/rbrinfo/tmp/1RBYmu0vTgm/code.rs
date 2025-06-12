fn eq(&self, other: &Slice<U>) -> bool {
        <[T] as PartialEq<Slice<U>>>::eq(self, other)
    }