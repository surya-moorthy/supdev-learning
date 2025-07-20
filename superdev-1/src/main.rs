use std::{fmt::Error, vec};

// not a macro yet

trait  Serialize {
    fn serialize(&self) -> Vec<u8>;
}
 
trait Deserialize {
    fn deserialize(v : Vec<u8>) -> Result<Swap,Error>;
}

#[derive(Debug)]
struct  Swap {
    qty1 : u32,
    qty2 : u32
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];
        v.extend_from_slice(&self.qty1.to_be_bytes());  // 1 -> [0,0,0,1]   for to_le_bytes 1 -> [1,0,0,0]
        v.extend_from_slice(&self.qty2.to_be_bytes());
        return v;
    }
}

impl Deserialize for Swap {


  fn deserialize(v : Vec<u8>) -> Result<Swap,Error> {
      if v.len() < 8 {
          return Err(Error);
      };    
      
      let qty1  = u32::from_be_bytes([v[0],v[1],v[2],v[3]]);
      let qty2  = u32::from_be_bytes([v[4],v[5],v[6],v[7]]);

      return Ok(
        Swap {
            qty1,
            qty2
        }
      );
  } 
}

fn main() {
    let s = Swap {
        qty1 : 4,
        qty2 : 5
    };
    let v = s.serialize();
    println!("{:?}",v);

    let dev = Swap::deserialize(v);
    println!("{:?}",dev);

}