fn main() {
    println!("Hello, world!");
}


#[allow(dead_code)]
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