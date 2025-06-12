#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn test_function6(_param0 :u8) {
    let _local0: once_cell::sync::OnceCell::<u8> = once_cell::sync::OnceCell::<u8>::with_value(_param0);
    let _local1_param0_helper1 = &(_local0);
    let _: std::option::Option::<&u8> = once_cell::sync::OnceCell::<u8>::get(_local1_param0_helper1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_u8(data, 0);
        test_function6(_param0);
    });
}
