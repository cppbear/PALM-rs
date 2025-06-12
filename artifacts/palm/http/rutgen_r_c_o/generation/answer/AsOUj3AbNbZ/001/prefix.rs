// Answer 0

#[test]
fn test_extensions_mut_default() {
    let mut request: Request<()> = Request::new(());
    let _extensions = request.extensions_mut();
}

#[test]
fn test_extensions_mut_with_some_extensions() {
    let mut request: Request<()> = Request::from_parts(
        Parts {
            extensions: Extensions {
                map: Some(Box::new(AnyMap::new())),
            },
            ..Default::default()
        },
        (),
    );
    let _extensions = request.extensions_mut();
}

#[test]
fn test_extensions_mut_with_none_extensions() {
    let mut request: Request<()> = Request::from_parts(
        Parts {
            extensions: Extensions {
                map: None,
            },
            ..Default::default()
        },
        (),
    );
    let _extensions = request.extensions_mut();
}

#[test]
fn test_extensions_mut_after_insertion() {
    let mut request: Request<()> = Request::new(());
    request.extensions_mut().insert("test_extension");
    let extensions = request.extensions();
}

