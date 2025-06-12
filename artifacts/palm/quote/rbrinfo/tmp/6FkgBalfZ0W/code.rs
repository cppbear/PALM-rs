fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            <T as RepAsIteratorExt>::quote_into_iter(*self)
        }