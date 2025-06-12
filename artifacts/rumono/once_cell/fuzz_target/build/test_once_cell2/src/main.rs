#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn test_function2(_param0 :u8) {
    let _local0: once_cell::sync::OnceCell::<u8> = once_cell::sync::OnceCell::<u8>::with_value(_param0);
    let _local1_param0_helper1 = &(_local0) as *const once_cell::sync::OnceCell::<u8>;
    let _local1: once_cell::unsync::OnceCell::<*const once_cell::sync::OnceCell::<u8>> = <once_cell::unsync::OnceCell::<*const once_cell::sync::OnceCell::<u8>> as std::convert::From::<*const once_cell::sync::OnceCell::<u8>>>::from(_local1_param0_helper1);
    let _: std::option::Option::<*const once_cell::sync::OnceCell::<u8>> = once_cell::unsync::OnceCell::<*const once_cell::sync::OnceCell::<u8>>::into_inner(_local1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_u8(data, 0);
        test_function2(_param0);
    });
}
