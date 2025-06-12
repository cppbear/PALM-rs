// Answer 0

#[test]
fn test_lazy_new_with_string_function() {
    fn f() -> String {
        String::from("Hello, world!")
    }
    let lazy_value = Lazy::new(f);
}

#[test]
fn test_lazy_new_with_integer_function() {
    fn f() -> i32 {
        42
    }
    let lazy_value = Lazy::new(f);
}

#[test]
fn test_lazy_new_with_empty_function() {
    fn f() -> () {
        ()
    }
    let lazy_value = Lazy::new(f);
}

#[test]
fn test_lazy_new_with_float_function() {
    fn f() -> f64 {
        3.14
    }
    let lazy_value = Lazy::new(f);
}

#[test]
fn test_lazy_new_with_struct_function() {
    #[derive(Debug)]
    struct MyStruct {
        value: i32,
    }
    fn f() -> MyStruct {
        MyStruct { value: 10 }
    }
    let lazy_value = Lazy::new(f);
}

