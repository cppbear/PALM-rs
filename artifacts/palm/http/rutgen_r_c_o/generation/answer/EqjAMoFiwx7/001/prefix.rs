// Answer 0

#[test]
fn test_from_static_valid_input() {
    let authority = Authority::from_static("example.com");
}

#[test]
fn test_from_static_valid_short_input() {
    let authority = Authority::from_static("a");
}

#[test]
fn test_from_static_valid_long_input() {
    let authority = Authority::from_static("a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z");
}

#[should_panic]
fn test_from_static_empty_string() {
    let authority = Authority::from_static("");
}

#[should_panic]
fn test_from_static_invalid_character() {
    let authority = Authority::from_static("example.com:80");
}

#[should_panic]
fn test_from_static_invalid_uri_character() {
    let authority = Authority::from_static("example.com?query");
}

