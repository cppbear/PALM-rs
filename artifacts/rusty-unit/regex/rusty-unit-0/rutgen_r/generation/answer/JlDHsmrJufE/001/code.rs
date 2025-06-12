// Answer 0

#[derive(Default)]
struct Hir {
    greed: bool,
}

impl Hir {
    fn swap_greed(&mut self, yes: bool) {
        self.greed = yes;
    }
}

struct ParserBuilder {
    hir: Hir,
}

impl Default for ParserBuilder {
    fn default() -> Self {
        ParserBuilder { hir: Hir::default() }
    }
}

impl ParserBuilder {
    pub fn swap_greed(&mut self, yes: bool) -> &mut ParserBuilder {
        self.hir.swap_greed(yes);
        self
    }
}

#[test]
fn test_swap_greed_enable() {
    let mut parser = ParserBuilder::default();
    parser.swap_greed(true);
    assert!(parser.hir.greed);
}

#[test]
fn test_swap_greed_disable() {
    let mut parser = ParserBuilder::default();
    parser.swap_greed(false);
    assert!(!parser.hir.greed);
}

#[test]
fn test_swap_greed_multiple_calls() {
    let mut parser = ParserBuilder::default();
    parser.swap_greed(true);
    assert!(parser.hir.greed);
    parser.swap_greed(false);
    assert!(!parser.hir.greed);
    parser.swap_greed(true);
    assert!(parser.hir.greed);
}

