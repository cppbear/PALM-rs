// Answer 0

#[test]
fn test_body_mut_string_non_empty() {
    let mut response: Response<String> = Response::new(String::from("test body"));
    response.body_mut().push_str(" appending text");
}

#[test]
fn test_body_mut_string_max_length() {
    let mut response: Response<String> = Response::new(String::from("a".repeat(1024)));
    response.body_mut().push_str(" appending max length");
}

#[test]
fn test_body_mut_vec_u8_non_empty() {
    let mut response: Response<Vec<u8>> = Response::new(vec![1, 2, 3]);
    response.body_mut().push(4);
}

#[test]
fn test_body_mut_vec_u8_max_length() {
    let mut response: Response<Vec<u8>> = Response::new(vec![0; 1024]);
    response.body_mut().push(5);
}

#[test]
fn test_body_mut_string_update() {
    let mut response: Response<String> = Response::new(String::from("initial"));
    let body_mut = response.body_mut();
    body_mut.clear();
    body_mut.push_str("updated");
}

#[test]
fn test_body_mut_vec_u8_update() {
    let mut response: Response<Vec<u8>> = Response::new(vec![10, 20]);
    let body_mut = response.body_mut();
    body_mut.clear();
    body_mut.push(30);
}

