fn property_set(
    name_map: &'static [(&'static str, &'static [(char, char)])],
    canonical: &'static str,
) -> Option<&'static [(char, char)]> {
    name_map
        .binary_search_by_key(&canonical, |x| x.0)
        .ok()
        .map(|i| name_map[i].1)
}