fn next(&mut self) -> Option<Self::Item> {
        match *self {
            LiteralIter::Empty => None,
            LiteralIter::Bytes(ref mut many) => {
                if many.is_empty() {
                    None
                } else {
                    let next = &many[0..1];
                    *many = &many[1..];
                    Some(next)
                }
            }
            LiteralIter::Single(ref mut one) => {
                if one.is_empty() {
                    None
                } else {
                    let next = &one[..];
                    *one = &[];
                    Some(next)
                }
            }
            LiteralIter::AC(ref mut lits) => {
                if lits.is_empty() {
                    None
                } else {
                    let next = &lits[0];
                    *lits = &lits[1..];
                    Some(&**next)
                }
            }
            LiteralIter::TeddySSSE3(ref mut lits) => {
                if lits.is_empty() {
                    None
                } else {
                    let next = &lits[0];
                    *lits = &lits[1..];
                    Some(&**next)
                }
            }
            LiteralIter::TeddyAVX2(ref mut lits) => {
                if lits.is_empty() {
                    None
                } else {
                    let next = &lits[0];
                    *lits = &lits[1..];
                    Some(&**next)
                }
            }
        }
    }