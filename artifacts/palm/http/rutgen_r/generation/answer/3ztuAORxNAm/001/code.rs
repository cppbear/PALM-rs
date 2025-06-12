// Answer 0

#[test]
fn test_extension_with_string() {
    use std::any::Any;
    use http::*;

    let req = Request::builder()
        .extension("My Extension".to_string())
        .body(())
        .unwrap();

    assert_eq!(req.extensions().get::<String>(), Some(&"My Extension".to_string()));
}

#[test]
fn test_extension_with_integer() {
    use std::any::Any;
    use http::*;

    let req = Request::builder()
        .extension(42)
        .body(())
        .unwrap();

    assert_eq!(req.extensions().get::<i32>(), Some(&42));
}

#[test]
fn test_extension_with_struct() {
    use std::any::Any;
    use http::*;

    #[derive(Clone)]
    struct MyStruct {
        data: &'static str,
    }

    let my_extension = MyStruct { data: "Some data" };

    let req = Request::builder()
        .extension(my_extension.clone())
        .body(())
        .unwrap();

    assert_eq!(req.extensions().get::<MyStruct>(), Some(&my_extension));
}

#[test]
fn test_extension_with_tuple() {
    use std::any::Any;
    use http::*;

    let tuple_extension = (1, "Tuple");

    let req = Request::builder()
        .extension(tuple_extension)
        .body(())
        .unwrap();

    assert_eq!(req.extensions().get::<(i32, &'static str)>(), Some(&(1, "Tuple")));
}

#[should_panic]
fn test_extension_with_invalid_type() {
    use std::any::Any;
    use http::*;

    // Attempting to retrieve an extension that was not added, should panic
    let req = Request::builder()
        .extension(100)
        .body(())
        .unwrap();

    assert_eq!(req.extensions().get::<&'static str>(), None);
}

