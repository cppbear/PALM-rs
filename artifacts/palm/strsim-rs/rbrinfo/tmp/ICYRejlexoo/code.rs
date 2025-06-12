pub fn generic_jaro_winkler<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f64
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let sim = generic_jaro(a, b);

    if sim > 0.7 {
        let prefix_length = a
            .into_iter()
            .take(4)
            .zip(b)
            .take_while(|(a_elem, b_elem)| a_elem == b_elem)
            .count();

        sim + 0.1 * prefix_length as f64 * (1.0 - sim)
    } else {
        sim
    }
}