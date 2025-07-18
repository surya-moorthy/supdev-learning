#[derive(Debug)] // debug trait as a custom derive macro 
struct User {
    name : String,
    age : u32   
}

fn main() {
    let u = User {
        name : "Surya".to_string(),
        age : 18 
    };  

    println!("{:?}",u); // declarative macro
}

#[post("/api/users")]  // attribute like procedural macro
fn get_users(){
    sqlx::query!("INSERT INTO USERS VALUES ()");  //function like procedural macro 
}
