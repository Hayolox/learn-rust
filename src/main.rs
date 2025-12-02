fn main() {
    println!("Hello, world!");
}


#[allow(dead_code)]
#[derive(Debug)]
enum Level {
    Admin,
    User,
    Guest
}


#[test]
fn test_enum() {
    let level = Level::Admin;
    match level {
        Level::Admin => println!("Admin"),
        Level::User => println!("User"),
        Level::Guest => println!("Guest"),
    }
}



struct  Person{
    name : String,
    age : u8,
    level : Level
}

impl Person {
    fn new(name: &str, age: u8, level: Level) -> Person {
        Person {
            name: name.to_string(),
            age,
            level,
        }
    }

    fn handle_year(&self ){
        const MAX_AGE : u8 = 100;
        if self.age > MAX_AGE {
            print!("Max")
        }else{
            print!("Not Max")
        }

    }
}

#[test]
fn test_void(){
   let jhon =  Person::new("John", 30, Level::Admin);
   println!("{} - {:?}", jhon.name, jhon.level);
   jhon.handle_year();
}