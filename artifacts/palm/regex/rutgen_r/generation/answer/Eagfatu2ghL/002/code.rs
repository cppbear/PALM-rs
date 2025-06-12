// Answer 0

#[derive(Debug)]
struct DummyCaptureName {
    name: String,
    index: usize,
}

mod ast {
    pub struct Group {
        pub kind: GroupKind,
    }

    pub enum GroupKind {
        CaptureIndex(usize),
        CaptureName(DummyCaptureName),
        NonCapturing(usize),
    }
}

mod hir {
    #[derive(Debug)]
    pub struct Group {
        pub kind: GroupKind,
        pub hir: Box<Hir>,
    }

    #[derive(Debug)]
    pub enum GroupKind {
        CaptureIndex(usize),
        CaptureName { name: String, index: usize },
        NonCapturing,
    }

    #[derive(Debug)]
    pub struct Hir;

    impl Hir {
        pub fn group(group: Group) -> Hir {
            Hir // just a placeholder
        }
    }
}

fn hir_group(group: &ast::Group, expr: hir::Hir) -> hir::Hir {
    let kind = match group.kind {
        ast::GroupKind::CaptureIndex(idx) => {
            hir::GroupKind::CaptureIndex(idx)
        }
        ast::GroupKind::CaptureName(ref capname) => {
            hir::GroupKind::CaptureName {
                name: capname.name.clone(),
                index: capname.index,
            }
        }
        ast::GroupKind::NonCapturing(_) => hir::GroupKind::NonCapturing,
    };
    hir::Hir::group(hir::Group {
        kind: kind,
        hir: Box::new(expr),
    })
}

#[test]
fn test_hir_group_capture_name() {
    let capname = DummyCaptureName {
        name: "test_name".to_string(),
        index: 0,
    };
    let group = ast::Group {
        kind: ast::GroupKind::CaptureName(capname),
    };
    let expr = hir::Hir; // some Hir expression
    let result = hir_group(&group, expr);

    if let hir::GroupKind::CaptureName { name, index } = {
        if let hir::Hir::group(hir::Group { kind, .. }) = result {
            kind
        } else {
            panic!("Expected Hir::group")
        }
    } {
        assert_eq!(name, "test_name");
        assert_eq!(index, 0);
    } else {
        panic!("Expected CatchGroupKind::CaptureName");
    }
}

#[test]
fn test_hir_group_capture_index() {
    let group = ast::Group {
        kind: ast::GroupKind::CaptureIndex(1),
    };
    let expr = hir::Hir; // some Hir expression
    let result = hir_group(&group, expr);
    
    if let hir::GroupKind::CaptureIndex(idx) = {
        if let hir::Hir::group(hir::Group { kind, .. }) = result {
            kind
        } else {
            panic!("Expected Hir::group")
        }
    } {
        assert_eq!(idx, 1);
    } else {
        panic!("Expected CatchGroupKind::CaptureIndex");
    }
} 

#[test]
fn test_hir_group_non_capturing() {
    let group = ast::Group {
        kind: ast::GroupKind::NonCapturing(0),
    };
    let expr = hir::Hir; // some Hir expression
    let result = hir_group(&group, expr);

    if let hir::GroupKind::NonCapturing = {
        if let hir::Hir::group(hir::Group { kind, .. }) = result {
            kind
        } else {
            panic!("Expected Hir::group")
        }
    } {
        // NonCapturing has no further assertions
    } else {
        panic!("Expected CatchGroupKind::NonCapturing");
    }
}

