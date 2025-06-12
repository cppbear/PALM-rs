// Answer 0

#[test]
fn test_var_with_sized_pointer() {
    struct MySizedStruct;
    let my_struct = MySizedStruct;
    let var = Var(&my_struct);
    let mut formatter = fmt::Formatter::new();
    let _result = var.fmt(&mut formatter);
}

#[test]
fn test_var_with_non_null_pointer() {
    struct MyStruct;
    let my_struct = MyStruct;
    let var = Var(&my_struct);
    let mut formatter = fmt::Formatter::new();
    let _result = var.fmt(&mut formatter);
}

#[should_panic]
fn test_var_with_null_pointer() {
    struct MyStruct;
    let var = Var(std::ptr::null::<MyStruct>());
    let mut formatter = fmt::Formatter::new();
    let _result = var.fmt(&mut formatter);
}

#[test]
fn test_var_with_different_sized_type() {
    struct AnotherSizedStruct;
    let another_struct = AnotherSizedStruct;
    let var = Var(&another_struct);
    let mut formatter = fmt::Formatter::new();
    let _result = var.fmt(&mut formatter);
}

