// Answer 0

#[test]
fn test_append_string_len_0() {
    let mut rng = rand::thread_rng();
    let alphanumeric = Alphanumeric;
    let mut string = String::new();
    alphanumeric.append_string(&mut rng, &mut string, 0);
}

#[test]
fn test_append_string_len_1() {
    let mut rng = rand::thread_rng();
    let alphanumeric = Alphanumeric;
    let mut string = String::new();
    alphanumeric.append_string(&mut rng, &mut string, 1);
}

#[test]
fn test_append_string_len_500() {
    let mut rng = rand::thread_rng();
    let alphanumeric = Alphanumeric;
    let mut string = String::new();
    alphanumeric.append_string(&mut rng, &mut string, 500);
}

#[test]
fn test_append_string_len_1000() {
    let mut rng = rand::thread_rng();
    let alphanumeric = Alphanumeric;
    let mut string = String::new();
    alphanumeric.append_string(&mut rng, &mut string, 1000);
}

#[test]
fn test_append_string_len_999() {
    let mut rng = rand::thread_rng();
    let alphanumeric = Alphanumeric;
    let mut string = String::new();
    alphanumeric.append_string(&mut rng, &mut string, 999);
}

