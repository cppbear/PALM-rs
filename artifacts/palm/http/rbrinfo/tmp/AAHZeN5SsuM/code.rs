fn to_raw_capacity(n: usize) -> usize {
    match n.checked_add(n / 3) {
        Some(n) => n,
        None => panic!(
            "requested capacity {} too large: overflow while converting to raw capacity",
            n
        ),
    }
}