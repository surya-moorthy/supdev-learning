use std::fmt::Error;
use serialize_macro::{SerializeNumberSturct,DeSerializeNumberSturct};
use serialize_macro_traits::{Serialize, Deserialize};

#[derive(Debug,SerializeNumberSturct, DeSerializeNumberSturct)]
struct Swap {
    qty_1: u32,
    qty_2: u32,
    qty_3: u32
}


fn main() {
    println!("Hello, world!");
    let s = Swap {
        qty_1: 1,
        qty_2: 2,
        qty_3: 1000
    };
    let bytes = s.serialize();
    let deserialized = Swap::deserialize(&bytes);
    println!("{:?} \n {:?}", bytes,deserialized);
}