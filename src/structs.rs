// Copy trait can't be used on string trait as it gets stored on heap for that u need to use clone trait ,
// In the case if there is not a string in struct then u can use copy trait because it only gets stored on stack

#[derive(Clone)]
struct User {
    active: bool,
    username: String,
}
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // fn print_str() {
    //     println!("Case of without &self as param");
    //  }
}

pub fn structs() {
    let user1 = User {
        active: true,
        username: "rohan".to_string(),
    };

    print_name(&user1);
    print!("User 1 username: {}", user1.username);
}

fn print_name(user1: &User) {
    print!("User 1 username: {}", user1.active);
}

pub fn rect_area() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    print!("The area of the rectangle is {}", rect.area());

    // Below is case of impl with struct without &self

    // Rect::print_str();
}
