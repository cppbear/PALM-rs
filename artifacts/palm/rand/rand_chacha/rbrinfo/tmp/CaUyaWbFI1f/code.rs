fn refill_wide_impl<Mach: Machine>(
    m: Mach,
    state: &mut ChaCha,
    drounds: u32,
    out: &mut [u32; BUFSZ],
) {
    let k = m.vec([0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574]);
    let b = m.unpack(state.b);
    let c = m.unpack(state.c);
    let mut x = State {
        a: Mach::u32x4x4::from_lanes([k, k, k, k]),
        b: Mach::u32x4x4::from_lanes([b, b, b, b]),
        c: Mach::u32x4x4::from_lanes([c, c, c, c]),
        d: d0123(m, state.d),
    };
    for _ in 0..drounds {
        x = round(x);
        x = undiagonalize(round(diagonalize(x)));
    }
    let kk = Mach::u32x4x4::from_lanes([k, k, k, k]);
    let sb = m.unpack(state.b);
    let sb = Mach::u32x4x4::from_lanes([sb, sb, sb, sb]);
    let sc = m.unpack(state.c);
    let sc = Mach::u32x4x4::from_lanes([sc, sc, sc, sc]);
    let sd = d0123(m, state.d);
    let results = Mach::u32x4x4::transpose4(x.a + kk, x.b + sb, x.c + sc, x.d + sd);
    out[0..16].copy_from_slice(&results.0.to_scalars());
    out[16..32].copy_from_slice(&results.1.to_scalars());
    out[32..48].copy_from_slice(&results.2.to_scalars());
    out[48..64].copy_from_slice(&results.3.to_scalars());
    state.d = add_pos(m, sd.to_lanes()[0], 4).into();
}