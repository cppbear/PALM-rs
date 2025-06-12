// Answer 0

#[test]
fn test_span_bool() {
    let input: &dyn IdentFragment = &true;
    let _ = input.span();
}

#[test]
fn test_span_str() {
    let input: &dyn IdentFragment = &"test";
    let _ = input.span();
}

#[test]
fn test_span_string() {
    let input: &dyn IdentFragment = &String::from("test");
    let _ = input.span();
}

#[test]
fn test_span_char() {
    let input: &dyn IdentFragment = &'a';
    let _ = input.span();
}

#[test]
fn test_span_u8() {
    let input: &dyn IdentFragment = &8u8;
    let _ = input.span();
}

#[test]
fn test_span_u16() {
    let input: &dyn IdentFragment = &16u16;
    let _ = input.span();
}

#[test]
fn test_span_u32() {
    let input: &dyn IdentFragment = &32u32;
    let _ = input.span();
}

#[test]
fn test_span_u64() {
    let input: &dyn IdentFragment = &64u64;
    let _ = input.span();
}

#[test]
fn test_span_u128() {
    let input: &dyn IdentFragment = &128u128;
    let _ = input.span();
}

#[test]
fn test_span_usize() {
    let input: &dyn IdentFragment = &size_of::<usize>();
    let _ = input.span();
}

#[test]
fn test_span_mut_bool() {
    let mut input = true;
    let input: &mut dyn IdentFragment = &mut input;
    let _ = input.span();
}

#[test]
fn test_span_mut_str() {
    let mut input = "test";
    let input: &mut dyn IdentFragment = &mut input;
    let _ = input.span();
}

#[test]
fn test_span_mut_string() {
    let mut input = String::from("test");
    let input: &mut dyn IdentFragment = &mut input;
    let _ = input.span();
}

#[test]
fn test_span_mut_char() {
    let mut input = 'a';
    let input: &mut dyn IdentFragment = &mut input;
    let _ = input.span();
}

#[test]
fn test_span_mut_u8() {
    let mut input = 8u8;
    let input: &mut dyn IdentFragment = &mut input;
    let _ = input.span();
}

#[test]
fn test_span_mut_u16() {
    let mut input = 16u16;
    let input: &mut dyn IdentFragment = &mut input;
    let _ = input.span();
}

#[test]
fn test_span_mut_u32() {
    let mut input = 32u32;
    let input: &mut dyn IdentFragment = &mut input;
    let _ = input.span();
}

#[test]
fn test_span_mut_u64() {
    let mut input = 64u64;
    let input: &mut dyn IdentFragment = &mut input;
    let _ = input.span();
}

#[test]
fn test_span_mut_u128() {
    let mut input = 128u128;
    let input: &mut dyn IdentFragment = &mut input;
    let _ = input.span();
}

#[test]
fn test_span_mut_usize() {
    let mut input = size_of::<usize>();
    let input: &mut dyn IdentFragment = &mut input;
    let _ = input.span();
}

