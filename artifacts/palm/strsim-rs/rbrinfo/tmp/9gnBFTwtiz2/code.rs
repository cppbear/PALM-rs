pub fn generic_hamming<Iter1, Iter2, Elem1, Elem2>(a: Iter1, b: Iter2) -> HammingResult
where
    Iter1: IntoIterator<Item = Elem1>,
    Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let (mut ita, mut itb) = (a.into_iter(), b.into_iter());
    let mut count = 0;
    loop {
        match (ita.next(), itb.next()) {
            (Some(x), Some(y)) => {
                if x != y {
                    count += 1;
                }
            }
            (None, None) => return Ok(count),
            _ => return Err(StrSimError::DifferentLengthArgs),
        }
    }
}