use std::fmt::Error;

pub trait Serialize {
   fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize {
    fn deserialize(&self) -> Result<Self,Error> where Self: Sized ;
}