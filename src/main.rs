mod tasks;
mod guessno;
mod structs;

fn main() {
    let myname = String::from("immutable reference");
    let mut firstname = String::from("Rohan");

    tasks::run(myname);
    tasks::append_text(&mut firstname);
    structs::structs();
    guessno::guess_number();
}
