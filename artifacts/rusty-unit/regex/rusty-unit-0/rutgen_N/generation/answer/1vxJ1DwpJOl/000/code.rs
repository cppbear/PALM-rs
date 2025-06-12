// Answer 0

#[derive(Debug)]
struct Hir {
    info: Info,
}

#[derive(Debug)]
struct Info {
    anchored_start: bool,
}

impl Info {
    fn is_anchored_start(&self) -> bool {
        self.anchored_start
    }
}

impl Hir {
    fn new(anchored_start: bool) -> Self {
        Hir {
            info: Info {
                anchored_start,
            },
        }
    }

    fn is_anchored_start(&self) -> bool {
        self.info.is_anchored_start()
    }
}

#[test]
fn test_is_anchored_start_true() {
    let hir = Hir::new(true);
    assert!(hir.is_anchored_start());
}

#[test]
fn test_is_anchored_start_false() {
    let hir = Hir::new(false);
    assert!(!hir.is_anchored_start());
}

