use std::fmt::Error;

pub trait Serialize {
   fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize {
    fn deserialize(size : &[u8]) -> Result<Self,Error> where Self: Sized ;
}