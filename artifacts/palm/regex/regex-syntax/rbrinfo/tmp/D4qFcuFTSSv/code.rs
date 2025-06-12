fn normalize(x: &str) -> String {
    let mut x = x.to_string();
    ucd_util::symbolic_name_normalize(&mut x);
    x
}