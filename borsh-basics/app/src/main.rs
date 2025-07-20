use std::fmt::Error;
use serialize_macro::{SerializeNumberStruct, DeSerializeNumberStruct};
use serialize_macro_traits::{Serialize, Deserialize};

#[derive(SerializeNumberStruct, DeSerializeNumberStruct)]
struct Swap {
    qty_1: u32,
    qty_2: u32,
    qty_3: u32
}

#[derive(SerializeNumberStruct, DeSerializeNumberStruct)]
// for unnamed struct 
struct Swap2(u32,u32,u32);

fn main() {
    println!("Hello, world!");
    let s = Swap {
        qty_1: 1,
        qty_2: 2,
        qty_3: 1000
    };
     
    let s2 = Swap2(3,2314,10313);

    let bytes = s.serialize();
    let bytes2 = s2.serialize();
    println!("{:?}", bytes);
    println!("{:?}",bytes2);
}