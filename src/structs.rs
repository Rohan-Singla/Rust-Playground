// Copy trait can't be used on string trait as it gets stored on heap for that u need to use clone trait , In the case if there is not a string in struct then u can use copy trait because it only gets stored on stack


#[derive(Clone)]
struct User {
    active: bool,
    sign_in_count: u64,
    username: String,
}

pub fn structs() {
    let user1 = User {
        active: true,
        sign_in_count: 1,
        username: "rohan".to_string(),
    };

    print_name(&user1);
    print!("User 1 username: {}", user1.username); 
}

fn print_name(user1: &User) {
    print!("User 1 username: {}", user1.active);
}
