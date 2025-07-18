struct Rect {
    width : u32,
    height : u32
}

struct Square {
    side : u32
}

// A trait in Rust is a language feature that defines shared behavior — a kind of interface or contract that types can implement.
// It specifies a set of methods (and possibly associated types or constants) that a type must provide in order to "implement" that trait.
  
trait Shape {
        fn area(self,message : String) -> (u32,String);
        fn perimeter(self) -> u32;
}

impl Shape for Rect {
    fn area(self,message : String) -> (u32,String) {
        return (self.width * self.height,message);
    }
    fn perimeter(self) -> u32 {
        return 2*(self.height + self.width);
    }
}

impl Shape for Square {
    fn area(self,message : String) -> (u32,String) {
        return (self.side * self.side,message);
    }
    fn perimeter(self) -> u32 {
        return 4 * self.side;
    }
}


fn main() {
    let r = Rect {
        width : 1,
        height : 4
    };
    let s = Square {
        side : 10
    };

    let (area , perimeter) = r.area("This is a rectangle".to_string());

    println!("{} {}",area,perimeter);
    print!("{:?} \n ",s.area("This is the square".to_string()));
}
