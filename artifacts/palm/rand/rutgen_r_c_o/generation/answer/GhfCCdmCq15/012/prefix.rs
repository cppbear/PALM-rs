// Answer 0

#[test]
fn test_append_string_case_1() {
    let slice: Vec<char> = vec!['a'; 200]; // length is 200 and all chars are single-byte.
    let rng = &mut rand::thread_rng();
    let mut result_string = String::new();
    let choose = Choose { slice: &slice, range: UniformUsize { low: 0, range: 1, thresh: 1 }, num_choices: NonZeroUsize::new(1).unwrap() };
    choose.append_string(rng, &mut result_string, 100);
}

#[test]
fn test_append_string_case_2() {
    let slice: Vec<char> = (0..200).map(|i| std::char::from_u32(i as u32).unwrap()).collect(); // length is 200 with distinct chars.
    let rng = &mut rand::thread_rng();
    let mut result_string = String::new();
    let choose = Choose { slice: &slice, range: UniformUsize { low: 0, range: 1, thresh: 1 }, num_choices: NonZeroUsize::new(1).unwrap() };
    choose.append_string(rng, &mut result_string, 100);
}

#[test]
fn test_append_string_case_3() {
    let slice: Vec<char> = vec!['b'; 200]; // length is 200 and all chars are single-byte.
    let rng = &mut rand::thread_rng();
    let mut result_string = String::new();
    let choose = Choose { slice: &slice, range: UniformUsize { low: 0, range: 1, thresh: 1 }, num_choices: NonZeroUsize::new(1).unwrap() };
    choose.append_string(rng, &mut result_string, 100);
}

#[test]
fn test_append_string_case_4() {
    let slice: Vec<char> = vec!['c'; 200]; // length is 200 and all chars are single-byte.
    let rng = &mut rand::thread_rng();
    let mut result_string = String::new();
    let choose = Choose { slice: &slice, range: UniformUsize { low: 0, range: 1, thresh: 1 }, num_choices: NonZeroUsize::new(1).unwrap() };
    choose.append_string(rng, &mut result_string, 100);
}

