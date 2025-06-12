#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

fn test_function261(_param0 :u16 ,_param1 :serde_json::value::Serializer) {
    let _local0 = <serde_json::Value as std::convert::From::<u16>>::from(_param0);
    let _local1: serde_json::Error = <serde_json::Error as serde::ser::Error>::custom(_local0);
    let _local2 = <std::io::Error as std::convert::From::<serde_json::Error>>::from(_local1);
    let _local3_param1_helper1 = &(_local2);
    let _: serde_json::Result::<serde_json::Value> = <serde_json::value::Serializer as serde::ser::Serializer>::collect_str(_param1, _local3_param1_helper1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 2 {return;}
        let _param0 = _to_u16(data, 0);
        let _param1 = serde_json::value::Serializer{};
        test_function261(_param0 ,_param1);
    });
}
