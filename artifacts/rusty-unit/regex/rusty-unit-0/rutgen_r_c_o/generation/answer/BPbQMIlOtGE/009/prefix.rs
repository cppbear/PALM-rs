// Answer 0

#[test]
fn test_replacen_case1() {
    let regex = Regex::new("abc").unwrap();
    let text = "abcdefabc";
    let limit = 0;
    let rep = "X";
    let _result = regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_case2() {
    let regex = Regex::new("a").unwrap();
    let text = "aaaaa";
    let limit = 0;
    let rep = "Z";
    let _result = regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_case3() {
    let regex = Regex::new("b").unwrap();
    let text = "abababab";
    let limit = 0;
    let rep = "Y";
    let _result = regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_case4() {
    let regex = Regex::new("x").unwrap();
    let text = "nothing to replace here";
    let limit = 0;
    let rep = "R";
    let _result = regex.replacen(text, limit, rep);
}

